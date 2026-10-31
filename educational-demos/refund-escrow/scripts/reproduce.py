"""Reproduce the educational lab locally; no RPC, wallet or third-party Python package."""
from pathlib import Path
import argparse
import datetime
import hashlib
import json
import os
import platform
import re
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
FORGE_VERSION = "1.7.1"
FORGE_COMMIT = "4072e48705af9d93e3c0f6e29e93b5e9a40caed8"
SEED = "0x0000000000000000000000000000000000000000000000000000000000000005"
FUZZ_TESTS = ("testFuzz_fixedPreservesBacking", "testFuzz_vulnerableCanBreakBacking")


def redact_home_paths(text):
    return re.subn(
        r"(?i)[A-Z]:[\\/]+Users[\\/]+[^\\/\s]+|/(?:home|Users)/[^/\s]+", "<HOME>", text
    )


def reserve_output(root, requested):
    """Reserve a new directory atomically; never reuse prior evidence."""
    target = (root / requested).resolve() if requested else root / "evidence"
    if requested:
        try:
            target.mkdir(parents=True, exist_ok=False)
            return target
        except FileExistsError:
            if not target.is_dir():
                raise ValueError("--output names an existing file, not a directory") from None
    else:
        target.mkdir(parents=True, exist_ok=True)
    prefix = datetime.datetime.now(datetime.timezone.utc).strftime("run-%Y%m%dT%H%M%S-")
    return Path(tempfile.mkdtemp(prefix=prefix, dir=target))


def display_output(output):
    try:
        return output.relative_to(ROOT).as_posix()
    except ValueError:
        return str(output)


def validated_config(config, profile):
    """Whitelist portable fields and reject a changed experiment budget."""
    expected = {
        "src": "src", "test": "test" if profile == "default" else "counterexamples",
        "solc": "0.8.24", "evm_version": "paris", "optimizer": True,
        "optimizer_runs": 200, "libs": [], "ffi": False,
    }
    for key, value in expected.items():
        if config.get(key) != value:
            raise RuntimeError(f"{profile} configuration mismatch: {key}")
    fuzz = config.get("fuzz", {})
    invariant = config.get("invariant", {})
    if fuzz.get("runs") != 256 or int(str(fuzz.get("seed", "0")), 0) != int(SEED, 16):
        raise RuntimeError(f"{profile} configuration mismatch: fuzz runs/seed")
    budget = {"runs": 128, "depth": 64, "fail_on_revert": True}
    if any(invariant.get(k) != v for k, v in budget.items()):
        raise RuntimeError(f"{profile} configuration mismatch: invariant budget")
    filters = ("match_test", "no_match_test", "match_contract", "no_match_contract", "match_path", "no_match_path")
    if any(config.get(key) for key in filters) or config.get("skip"):
        raise RuntimeError(f"{profile} configuration contains test filters")
    if config.get("eth_rpc_url") or config.get("fork_block_number") is not None:
        raise RuntimeError(f"{profile} configuration requests RPC/fork access")
    return {
        **expected, "fuzz": {"runs": fuzz["runs"], "seed": SEED},
        "invariant": {k: invariant[k] for k in budget},
    }


def validate_suite(text):
    if not re.search(r"13 tests passed, 0 failed, 0 skipped", text):
        raise RuntimeError("Default suite did not report all 13 expected tests")
    for name in FUZZ_TESTS:
        if not re.search(r"\[PASS\] " + name + r"\([^\n]*\) \(runs: 256,", text):
            raise RuntimeError(f"Missing 256-case fuzz result: {name}")
    if not re.search(r"runs: 128, calls: 8192, reverts: 0", text):
        raise RuntimeError("Stateful campaign did not reach its configured call budget")


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--forge", default="forge", help="Forge 1.7.1 executable or PATH name")
    parser.add_argument("--output", help="Output directory; an existing directory receives a new unique child")
    args = parser.parse_args(argv)
    executable = shutil.which(args.forge)
    if executable is None:
        print("Forge not found. Install Foundry 1.7.1, or pass --forge.", file=sys.stderr)
        return 2
    try:
        output = reserve_output(ROOT, args.output)
    except (OSError, ValueError) as exc:
        print(f"Cannot reserve evidence directory: {exc}", file=sys.stderr)
        return 2
    env = os.environ.copy()
    for key in list(env):
        if key.startswith(("FOUNDRY_", "DAPP_")):
            del env[key]
    env["NO_COLOR"] = "1"
    env["FOUNDRY_FUZZ_SEED"] = SEED
    inputs = {}
    steps = []
    configs = {}

    def run(name, argv, profile="default", expected=0, config=False):
        local_env = env.copy()
        local_env["FOUNDRY_PROFILE"] = profile
        result = subprocess.run(
            [executable, *argv], cwd=ROOT, env=local_env, capture_output=True, timeout=300
        )
        text = (result.stdout + result.stderr).decode("utf-8", errors="replace")
        if config:
            # Full Forge config can contain paths or credentials: publish only a whitelist.
            text = "Configuration command failed" if result.returncode else json.dumps(
                validated_config(json.loads(result.stdout), profile), indent=2
            )
            if not result.returncode:
                configs[profile] = json.loads(text)
        text, redacted = redact_home_paths(text)
        (output / (name + ".txt")).write_text(text, encoding="utf-8")
        steps.append({
            "name": name, "command": "forge " + " ".join(argv), "profile": profile,
            "exit_code": result.returncode, "expected_exit_code": expected,
            "redacted_home_path_occurrences": redacted,
        })
        if result.returncode != expected:
            raise RuntimeError(f"{name}: expected exit {expected}, received {result.returncode}")
        return text

    status = "FAILED"
    error = None
    try:
        for folder in ["src", "test", "counterexamples", "scripts"]:
            for path in sorted((ROOT / folder).rglob("*")):
                if path.suffix in {".sol", ".py"}:
                    inputs[path.relative_to(ROOT).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
        inputs["foundry.toml"] = hashlib.sha256((ROOT / "foundry.toml").read_bytes()).hexdigest()
        version = run("toolchain", ["--version"])
        if not re.search(r"\bVersion:\s*" + re.escape(FORGE_VERSION) + r"\b", version) or FORGE_COMMIT not in version:
            raise RuntimeError("Forge version/commit differs from the recorded 1.7.1 release")
        run("config-default", ["config", "--json"], config=True)
        run("format", ["fmt", "--check"])
        run("build", ["build"])
        validate_suite(run("default-suite", ["test", "-vvv"]))
        run("config-counterexample", ["config", "--json"], profile="counterexample", config=True)
        counter = run(
            "intentional-counterexample",
            ["test", "--match-contract", "VulnerableSolvencyCounterexample", "-vvv"],
            profile="counterexample", expected=1,
        )
        if "[FAIL: INVARIANT_BROKEN: underbacked credits]" not in counter:
            raise RuntimeError("Nonzero exit was not the intended safety-property failure")
        if "0 tests passed, 1 failed, 0 skipped" not in counter:
            raise RuntimeError("Counterexample did not execute its one safety test")
        status = "PASS"
    except Exception as exc:
        error, _ = redact_home_paths(str(exc))
    now = datetime.datetime.now(datetime.timezone.utc)
    record = {
        "review_date": now.date().isoformat(), "captured_at_utc": now.isoformat(),
        "artifact_type": "Original educational demonstration of a known vulnerability class",
        "overall_experiment": status, "error": error,
        "platform": {"system": platform.system(), "architecture": platform.machine()},
        "compiler": "Solc 0.8.24 required; see effective_config for verified settings",
        "evm": "paris", "seed": SEED, "library_dependencies": [],
        "rpc_or_production_transactions": False, "effective_config": configs,
        "steps": steps, "execution_input_sha256": inputs,
    }
    (output / "RUN.json").write_text(json.dumps(record, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"overall_experiment": status, "output": display_output(output)}))
    return 0 if status == "PASS" else 1


if __name__ == "__main__":
    sys.exit(main())
