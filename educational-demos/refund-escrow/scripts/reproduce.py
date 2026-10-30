"""Run the educational experiment locally; no RPC, wallet, or third-party Python package."""
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

ROOT = Path(__file__).resolve().parents[1]
SEED = "0x0000000000000000000000000000000000000000000000000000000000000005"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--forge", default="forge", help="Existing Forge executable or PATH name")
    parser.add_argument("--output", help="New evidence directory; existing directories are preserved")
    args = parser.parse_args()
    executable = shutil.which(args.forge)
    if executable is None:
        raise SystemExit("Forge not found. Install Foundry and put forge on PATH, or pass --forge.")
    output = (
        ROOT / args.output
        if args.output
        else ROOT / "evidence" / datetime.datetime.now(datetime.timezone.utc).strftime("run-%Y%m%dT%H%M%SZ")
    )
    output.mkdir(parents=True, exist_ok=False)
    env = os.environ.copy()
    # Prevent inherited FOUNDRY_* overrides from silently changing this experiment.
    for key in list(env):
        if key.startswith(("FOUNDRY_", "DAPP_")):
            del env[key]
    env["NO_COLOR"] = "1"
    env["FOUNDRY_FUZZ_SEED"] = SEED
    inputs = {}
    for folder in ["src", "test", "counterexamples", "scripts"]:
        for path in sorted((ROOT / folder).rglob("*")):
            if path.suffix in {".sol", ".py"}:
                inputs[path.relative_to(ROOT).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
    inputs["foundry.toml"] = hashlib.sha256((ROOT / "foundry.toml").read_bytes()).hexdigest()
    steps = []

    def run(name, argv, profile="default", expected=0):
        local_env = env.copy()
        local_env["FOUNDRY_PROFILE"] = profile
        result = subprocess.run(
            [executable, *argv], cwd=ROOT, env=local_env, capture_output=True
        )
        text = (result.stdout + result.stderr).decode("utf-8", errors="replace")
        # Source paths are portable; any tool diagnostic revealing a home directory is redacted.
        text, redacted = re.subn(
            r"(?i)[A-Z]:[\\/]+Users[\\/]+[^\\/\s]+|/home/[^/\s]+", "<HOME>", text
        )
        (output / (name + ".txt")).write_text(text, encoding="utf-8")
        step = {
            "name": name,
            "command": "forge " + " ".join(argv),
            "profile": profile,
            "exit_code": result.returncode,
            "expected_exit_code": expected,
            "redacted_home_path_occurrences": redacted,
        }
        steps.append(step)
        if result.returncode != expected:
            raise RuntimeError(f"{name}: expected exit {expected}, received {result.returncode}")
        return text

    status = "FAILED"
    error = None
    try:
        version = run("toolchain", ["--version"])
        run("format", ["fmt", "--check"])
        run("build", ["build"])
        suite = run("default-suite", ["test", "-vvv"])
        if not re.search(r"13 tests passed, 0 failed, 0 skipped", suite):
            raise RuntimeError("Default suite did not report all 13 expected tests")
        if not re.search(r"runs: 128, calls: 8192, reverts: 0", suite):
            raise RuntimeError("Stateful campaign did not reach its configured call budget")
        counter = run(
            "intentional-counterexample",
            ["test", "--match-contract", "VulnerableSolvencyCounterexample", "-vvv"],
            profile="counterexample",
            expected=1,
        )
        if "[FAIL: INVARIANT_BROKEN: underbacked credits]" not in counter:
            raise RuntimeError("Nonzero exit was not the intended safety-property failure")
        if "0 tests passed, 1 failed, 0 skipped" not in counter:
            raise RuntimeError("Counterexample did not execute its one safety test")
        status = "PASS"
    except Exception as exc:
        error = str(exc)
    record = {
        "review_date": "2026-10-30",
        "captured_at_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "artifact_type": "Original educational demonstration of a known vulnerability class",
        "overall_experiment": status,
        "error": error,
        "platform": {"system": platform.system(), "architecture": platform.machine()},
        "compiler": "Solc 0.8.24, pinned in foundry.toml",
        "evm": "paris",
        "seed": SEED,
        "library_dependencies": [],
        "rpc_or_production_transactions": False,
        "steps": steps,
        "execution_input_sha256": inputs,
    }
    (output / "RUN.json").write_text(json.dumps(record, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"overall_experiment": status, "output": output.relative_to(ROOT).as_posix()}))
    return 0 if status == "PASS" else 1


if __name__ == "__main__":
    sys.exit(main())
