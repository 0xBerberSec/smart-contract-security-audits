"""Run unchanged historical harnesses against a block-pinned local Anvil fork."""
from pathlib import Path
import datetime
import hashlib
import json
import os
import platform
import re
import shutil
import socket
import subprocess
import tempfile
import time
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
BLOCK = 12674080
CHAIN = 31612
BLOCK_HASH = "0x202a0c344bdf6230ba40ec63b09a9dc51de8342a58844fc1f202b7f36ed5f913"
COMMIT = "4072e48705af9d93e3c0f6e29e93b5e9a40caed8"
LOCAL = "http://127.0.0.1:8546"
SEED = "0x0000000000000000000000000000000000000000000000000000000000000005"
SUITES = {
    "MezoPoolBoundedStatefulFuzz": ["testFuzzQuoteBounded"],
    "MezoPoolAccountingInvariants": [
        "testNoFreeOutputFromZeroInputReadPath",
        "testReservesCoveredByTokenBalancesAndSupply",
    ],
    "MezoRouterSlippageAndDeadline": [
        "testEmptyPathRejected", "testExpiredDeadlineRejectedBeforeTransfer",
    ],
    "MezoUniversalRouterFork": [
        "testExpiredDeadlineRejected", "testInvalidCommandRejected",
    ],
}


def rpc(url, method, params):
    body = json.dumps({"jsonrpc": "2.0", "id": 1, "method": method, "params": params})
    request = urllib.request.Request(
        url, data=body.encode(), headers={"Content-Type": "application/json"}
    )
    with urllib.request.urlopen(request, timeout=15) as response:
        result = json.load(response)
    if "error" in result:
        raise RuntimeError("RPC returned an error for " + method)
    return result["result"]


def clean(text, upstream):
    text = text.replace(upstream, "<UPSTREAM_RPC>") if upstream else text
    text = re.sub(r"https?://[^\s\"'<>]+", "<URL>", text)
    return re.sub(r"(?i)[A-Z]:[\\/]+Users[\\/]+[^\\/\s]+|/(?:home|Users)/[^/\s]+", "<HOME>", text)


def main():
    output = Path(tempfile.mkdtemp(prefix="mezo-reproduction-"))
    upstream = os.environ.get("MEZO_RPC_URL", "")
    record = {
        "captured_at_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "platform": platform.system(), "overall": "BLOCKED", "suites": [],
        "fork": {"block": BLOCK, "chain_id": CHAIN},
        "public_transactions_sent_by_runner": 0,
        "source_sha256": {},
        "limits": ["fixed-state conditional quote assertions", "Router checks only low-level failure"],
    }
    node = None
    try:
        for folder in ("test", "scripts"):
            for path in sorted((ROOT / folder).rglob("*")):
                if path.is_file() and path.suffix in (".sol", ".py"):
                    record["source_sha256"][path.relative_to(ROOT).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
        record["source_sha256"]["foundry.toml"] = hashlib.sha256((ROOT / "foundry.toml").read_bytes()).hexdigest()
        if not upstream:
            raise RuntimeError("MEZO_RPC_URL is required; use a read-only Mezo archive-capable RPC")
        forge = os.environ.get("FORGE_BIN") or shutil.which("forge")
        anvil = os.environ.get("ANVIL_BIN") or shutil.which("anvil")
        if not forge or not anvil:
            raise RuntimeError("Forge and Anvil must already be installed")
        for name, binary in (("forge", forge), ("anvil", anvil)):
            version = subprocess.run([binary, "--version"], capture_output=True, text=True, timeout=15)
            text = version.stdout + version.stderr
            if version.returncode or not re.search(r"\b1\.7\.1\b", text) or COMMIT not in text:
                raise RuntimeError(name + " must be Foundry 1.7.1 at the recorded commit")
            record[name + "_version"] = clean(text, upstream)
        config = subprocess.run([forge, "config", "--json"], cwd=ROOT,
                                capture_output=True, text=True, timeout=15)
        if config.returncode:
            raise RuntimeError("Unable to read effective Foundry configuration")
        settings = json.loads(config.stdout)
        expected = {"solc": "0.8.24", "evm_version": "london", "ffi": False,
                    "test": "test", "src": "src"}
        if any(settings.get(key) != value for key, value in expected.items()):
            raise RuntimeError("Effective Foundry configuration differs from the recorded project")
        record["effective_config"] = {key: settings[key] for key in expected}
        if int(rpc(upstream, "eth_chainId", []), 16) != CHAIN:
            raise RuntimeError("Upstream chain ID differs from Mezo 31612")
        header = rpc(upstream, "eth_getBlockByNumber", [hex(BLOCK), False])
        if not header or int(header["number"], 16) != BLOCK:
            raise RuntimeError("Historical fork block is unavailable")
        if header["hash"] != BLOCK_HASH:
            raise RuntimeError("Historical block hash differs from the recorded Mezo block")
        record["fork"].update({"hash": header["hash"], "timestamp": int(header["timestamp"], 16)})
        with socket.socket() as probe:
            if probe.connect_ex(("127.0.0.1", 8546)) == 0:
                raise RuntimeError("Local port 8546 is already in use; existing services are not reused or stopped")
        args = [anvil, "--fork-url", upstream, "--fork-block-number", str(BLOCK),
                "--host", "127.0.0.1", "--port", "8546", "--accounts", "0", "--quiet"]
        node = subprocess.Popen(args, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        ready = False
        for _ in range(60):
            if node.poll() is not None:
                raise RuntimeError("Local Anvil exited during startup")
            try:
                ready = int(rpc(LOCAL, "eth_chainId", []), 16) == CHAIN
                if ready:
                    break
            except Exception:
                pass
            time.sleep(0.5)
        if not ready:
            raise RuntimeError("Local Anvil did not become ready")
        local_header = rpc(LOCAL, "eth_getBlockByNumber", [hex(BLOCK), False])
        if not local_header or local_header["hash"] != header["hash"]:
            raise RuntimeError("Local fork block hash differs from the upstream header")
        addresses = [
            "0x52e604c44417233b6ccedddc0d640a405caacefb",
            "0xed812aec0fecc8fd882ac3eccc43f3aa80a6c356",
            "0x10906a9e9215939561597b4c8e4b98f93c02031a",
            "0xac93a67f15342e8ef611396b13919204c8799cd4",
            "0xd97576d5fed11c7c31095d4fd0af612dc736679b",
        ]
        for address in addresses:
            if rpc(LOCAL, "eth_getCode", [address, hex(BLOCK)]) in ("0x", "0x0"):
                raise RuntimeError("Missing deployed code for a selected fork target")
        env = os.environ.copy()
        env["MEZO_RPC_URL"] = LOCAL
        env["NO_COLOR"] = "1"
        for suite, tests in SUITES.items():
            args = ["test", "--fork-url", LOCAL, "--fork-block-number", str(BLOCK),
                    "--match-contract", "^" + suite + "$", "--fuzz-runs", "1024",
                    "--fuzz-seed", SEED, "-vv"]
            result = subprocess.run([forge, *args], cwd=ROOT, env=env,
                                    capture_output=True, text=True, timeout=180)
            text = clean(result.stdout + result.stderr, upstream)
            (output / (suite + ".txt")).write_text(text, encoding="utf-8")
            passed = result.returncode == 0 and all(
                re.search(r"\[PASS\]\s+" + re.escape(test) + r"\(", text) for test in tests
            )
            if suite == "MezoPoolBoundedStatefulFuzz":
                passed = passed and bool(re.search(r"runs:\s*1024\b", text))
            record["suites"].append({"suite": suite, "command": "forge " + " ".join(args),
                                      "exit_code": result.returncode, "expected_tests": tests,
                                      "status": "PASS" if passed else "FAILED"})
            if not passed:
                record["overall"] = "FAILED"
                raise RuntimeError("Suite did not reach every recorded passing assertion: " + suite)
        record["overall"] = "PASS"
    except Exception as exc:
        record["error"] = clean(str(exc), upstream)
    finally:
        if node is not None:
            node.terminate()
            try:
                node.wait(timeout=10)
            except subprocess.TimeoutExpired:
                node.kill()
                node.wait(timeout=10)
        (output / "RUN.json").write_text(json.dumps(record, indent=2) + "\n", encoding="utf-8")
    print(record["overall"] + ": " + str(output))
    if record.get("error"):
        print(record["error"])
    return 0 if record["overall"] == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
