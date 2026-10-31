"""Runner regression tests with mocked Forge output; not Solidity execution evidence."""
from pathlib import Path
import contextlib
import importlib.util
import io
import json
import subprocess
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("reproduce", Path(__file__).parents[1] / "reproduce.py")
runner = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(runner)

VERSION = f"forge Version: {runner.FORGE_VERSION}\nCommit SHA: {runner.FORGE_COMMIT}\n"
SUITE = "\n".join([
    "[PASS] testFuzz_fixedPreservesBacking(uint96,uint96) (runs: 256, gas: 1)",
    "[PASS] testFuzz_vulnerableCanBreakBacking(uint96) (runs: 256, gas: 1)",
    "[PASS] invariant_balanceCoversCredits() (runs: 128, calls: 8192, reverts: 0)",
    "13 tests passed, 0 failed, 0 skipped",
])
COUNTER = "[FAIL: INVARIANT_BROKEN: underbacked credits]\n0 tests passed, 1 failed, 0 skipped"


def config(profile="default"):
    return {
        "src": "src", "test": "test" if profile == "default" else "counterexamples",
        "solc": "0.8.24", "evm_version": "paris", "optimizer": True,
        "optimizer_runs": 200, "libs": [], "ffi": False,
        "fuzz": {"runs": 256, "seed": "0x5"},
        "invariant": {"runs": 128, "depth": 64, "fail_on_revert": True},
    }


class RunnerTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / "lab with spaces"
        self.root.mkdir()
        for folder in ("src", "test", "counterexamples", "scripts"):
            (self.root / folder).mkdir()
        (self.root / "foundry.toml").write_text("# test fixture\n", encoding="utf8")
        self.outputs = io.StringIO()
        self.mutate_config = lambda c: c
        self.version = VERSION
        self.suite = SUITE
        self.counter = COUNTER

    def fake_forge(self, args, **kwargs):
        profile = kwargs["env"]["FOUNDRY_PROFILE"]
        argv = args[1:]
        code = 0
        if argv == ["--version"]:
            text = self.version
        elif argv == ["config", "--json"]:
            text = json.dumps(self.mutate_config(config(profile)))
        elif argv == ["test", "-vvv"]:
            text = self.suite
        elif argv[:2] == ["test", "--match-contract"]:
            text, code = self.counter, 1
        else:
            text = ""
        return subprocess.CompletedProcess(args, code, text.encode(), b"")

    def invoke(self, output=None):
        args = [] if output is None else ["--output", str(output)]
        with patch.object(runner, "ROOT", self.root), patch.object(runner.shutil, "which", return_value="forge"):
            with patch.object(runner.subprocess, "run", side_effect=self.fake_forge) as calls:
                with contextlib.redirect_stdout(self.outputs), contextlib.redirect_stderr(self.outputs):
                    code = runner.main(args)
        return code, calls

    def test_relative_missing_directory(self):
        code, _ = self.invoke("nested/new evidence")
        self.assertEqual(code, 0)
        self.assertEqual(json.loads((self.root / "nested/new evidence/RUN.json").read_text())["overall_experiment"], "PASS")

    def test_absolute_external_path_with_spaces(self):
        outside = Path(self.temp.name) / "external evidence with spaces"
        code, _ = self.invoke(outside)
        self.assertEqual(code, 0)
        self.assertTrue(Path(json.loads(self.outputs.getvalue())["output"]).samefile(outside))
        self.assertTrue((outside / "RUN.json").is_file())

    def test_existing_directory_preserves_evidence_and_uses_unique_children(self):
        target = self.root / "prior evidence"
        target.mkdir()
        sentinel = target / "RUN.json"
        sentinel.write_bytes(b"previous evidence")
        self.assertEqual(self.invoke(target)[0], 0)
        self.assertEqual(self.invoke(target)[0], 0)
        self.assertEqual(sentinel.read_bytes(), b"previous evidence")
        self.assertEqual(len(list(target.glob("run-*/RUN.json"))), 2)

    def test_existing_file_rejected_without_running_forge(self):
        target = self.root / "file.txt"
        target.write_bytes(b"preserve")
        code, calls = self.invoke(target)
        self.assertEqual(code, 2)
        calls.assert_not_called()
        self.assertEqual(target.read_bytes(), b"preserve")

    def test_default_output_is_always_unique(self):
        self.assertEqual(self.invoke()[0], 0)
        self.assertEqual(self.invoke()[0], 0)
        self.assertEqual(len(list((self.root / "evidence").glob("run-*/RUN.json"))), 2)

    def test_changed_fuzz_budget_rejected(self):
        def changed(c):
            c["fuzz"]["runs"] = 1
            return c
        self.mutate_config = changed
        self.assertEqual(self.invoke("reduced-budget")[0], 1)

    def test_changed_invariant_budget_rejected(self):
        def changed(c):
            c["invariant"]["depth"] = 1
            return c
        self.mutate_config = changed
        self.assertEqual(self.invoke("reduced-invariant")[0], 1)

    def test_config_cannot_include_test_filter_or_rpc(self):
        for key,value in [("match_test", "onlyOne"), ("eth_rpc_url", "https://example.invalid")]:
            with self.subTest(key=key):
                c = config()
                c[key] = value
                with self.assertRaises(RuntimeError):
                    runner.validated_config(c, "default")

    def test_wrong_forge_version_rejected(self):
        self.version = "forge Version: 9.0.0"
        self.assertEqual(self.invoke("wrong-version")[0], 1)

    def test_missing_fuzz_cases_rejected_even_when_total_is_13(self):
        self.suite = SUITE.replace("runs: 256,", "runs: 1,")
        self.assertEqual(self.invoke("short-fuzz")[0], 1)

    def test_wrong_counterexample_reason_rejected(self):
        self.counter = COUNTER.replace("INVARIANT_BROKEN: underbacked credits", "setup error")
        self.assertEqual(self.invoke("wrong-reason")[0], 1)

    def test_effective_configuration_does_not_publish_unrelated_values(self):
        c = config()
        c["etherscan_api_key"] = "synthetic-unused-value"
        self.assertNotIn("etherscan_api_key", runner.validated_config(c, "default"))

    def test_configuration_error_produces_failed_record(self):
        self.mutate_config = lambda c: {**c, "solc": "0.8.25"}
        self.assertEqual(self.invoke("bad-compiler")[0], 1)
        record = json.loads((self.root / "bad-compiler/RUN.json").read_text())
        self.assertEqual(record["overall_experiment"], "FAILED")
        self.assertIn("solc", record["error"])


if __name__ == "__main__":
    unittest.main()
