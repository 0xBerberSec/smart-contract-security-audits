#!/usr/bin/env python3
"""Reproduce the two public GMX EVM adaptations in an isolated checkout.

This script never uses the Audit-OS checkout and never talks to a chain.  It
clones the official public repository, pins the audited revision, installs from
the repository lockfile, compiles, copies the two disclosure-bounded tests, and
then runs each file.  A non-zero command or an unexpected test count is a
failure; no historical transcript is converted into a PASS.
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

REPO_URL = "https://github.com/gmx-io/gmx-synthetics.git"
COMMIT = "10cfbce6144ade8e9d5cd8c811573955bcd1c132"
TESTS = (
    "external-handler-safe-controls.spec.ts",
    "vault-accounting-observations.spec.ts",
)


def run(cmd: list[str], cwd: Path, log: Path) -> int:
    with log.open("a", encoding="utf-8") as stream:
        stream.write("$ " + " ".join(cmd) + "\n")
        proc = subprocess.run(cmd, cwd=cwd, stdout=stream, stderr=subprocess.STDOUT, text=True)
        stream.write(f"[exit {proc.returncode}]\n")
        return proc.returncode


def require(tool: str) -> None:
    if shutil.which(tool) is None:
        raise RuntimeError(f"required tool not found on PATH: {tool}")


def version(tool: str) -> str:
    executable = tool
    if os.name == "nt" and tool in {"npm", "npx", "yarn"}:
        executable = tool + ".cmd"
    try:
        return subprocess.check_output([executable, "--version"], text=True, stderr=subprocess.STDOUT).strip()
    except Exception as exc:
        return f"unavailable: {exc}"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--workdir", type=Path, help="preserve this isolated workspace")
    args = parser.parse_args()

    for tool in ("git", "node", "npm", "npx"):
        require(tool)

    script_root = Path(__file__).resolve().parents[1]
    source_tests = script_root / "evm" / "original-gmx-tests"
    missing = [name for name in TESTS if not (source_tests / name).is_file()]
    if missing:
        raise RuntimeError("public adaptation missing: " + ", ".join(missing))

    owned_temp = args.workdir is None
    workspace = Path(tempfile.mkdtemp(prefix="gmx-evm-reproduction-")) if owned_temp else args.workdir
    workspace.mkdir(parents=True, exist_ok=True)
    log = workspace / "reproduction.log"
    summary = {
        "repository": REPO_URL,
        "commit_expected": COMMIT,
        "workspace": str(workspace),
        "tests": list(TESTS),
        "status": "BLOCKED",
        "commands": [],
        "tool_versions": {tool: version(tool) for tool in ("git", "node", "npm", "npx", "yarn")},
    }

    try:
        summary["commands"].append(f"git clone --filter=blob:none {REPO_URL} gmx-synthetics")
        if run(["git", "clone", "--filter=blob:none", REPO_URL, "gmx-synthetics"], workspace, log):
            raise RuntimeError("git clone failed")
        checkout = workspace / "gmx-synthetics"
        summary["commands"].append(f"git checkout --detach {COMMIT}")
        if run(["git", "checkout", "--detach", COMMIT], checkout, log):
            raise RuntimeError("checkout failed")
        actual = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=checkout, text=True).strip()
        summary["commit_actual"] = actual
        if actual != COMMIT:
            raise RuntimeError(f"commit mismatch: {actual}")

        lockfile = checkout / "package-lock.json"
        yarn_lock = checkout / "yarn.lock"
        if lockfile.exists():
            install = ["npm", "ci"]
        elif yarn_lock.exists():
            require("yarn")
            # Python cannot execute a PowerShell shim directly on Windows;
            # yarn.cmd is the executable entry point exposed by Node installs.
            install = ["yarn.cmd" if os.name == "nt" else "yarn", "install", "--frozen-lockfile"]
        else:
            install = ["npm", "install"]
        summary["commands"].append(" ".join(install))
        if run(install, checkout, log):
            raise RuntimeError("dependency installation failed")
        npx = "npx.cmd" if os.name == "nt" else "npx"
        summary["commands"].append(f"{npx} hardhat compile")
        if run([npx, "hardhat", "compile"], checkout, log):
            raise RuntimeError("clean Hardhat compilation failed")

        guardian = checkout / "test" / "guardian"
        guardian.mkdir(parents=True, exist_ok=True)
        for name in TESTS:
            shutil.copy2(source_tests / name, guardian / name)

        counts = {}
        for name in TESTS:
            test_log = workspace / (name + ".log")
            summary["commands"].append(f"{npx} hardhat test test/guardian/{name}")
            code = run([npx, "hardhat", "test", f"test/guardian/{name}"], checkout, test_log)
            text = test_log.read_text(encoding="utf-8", errors="replace")
            marker = "2 passing" in text
            counts[name] = {"exit_code": code, "two_tests_reported": marker, "log": str(test_log)}
            if code != 0 or not marker:
                raise RuntimeError(f"unexpected result for {name}; inspect {test_log}")

        summary["results"] = counts
        summary["status"] = "FRESH_PASS"
        print(json.dumps(summary, indent=2))
        return 0
    except Exception as exc:
        summary["error"] = str(exc)
        print(json.dumps(summary, indent=2))
        return 2
    finally:
        (workspace / "summary.json").write_text(json.dumps(summary, indent=2), encoding="utf-8")
        if owned_temp:
            print(f"isolated workspace retained at: {workspace}")


if __name__ == "__main__":
    sys.exit(main())
