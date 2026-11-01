"""Separate complementary experiment; never changes the historical runner or tests."""
from pathlib import Path
import argparse
import datetime
import hashlib
import json
import os
import re
import shutil
import socket
import subprocess
import tempfile
import time
import urllib.request

ROOT = Path(__file__).resolve().parents[1] / "supplemental"
COMMIT = "4072e48705af9d93e3c0f6e29e93b5e9a40caed8"
BLOCK = 12674080
CHAIN = 31612
BLOCK_HASH = "0x202a0c344bdf6230ba40ec63b09a9dc51de8342a58844fc1f202b7f36ed5f913"
SEED = "0x" + "0" * 63 + "1"
NAMES = {
    "testFuzzMaterialQuotesAllPairs", "testQuoteGridCoverage",
    "testVolatileQuoteMonotonicity", "testNativeIdentityAndMinimumLpHolder",
    "testRouterExactDeadlineAndPath", "testRouterExactSlippageWithQuoteControl",
    "testUniversalRouterPositiveEmptyExecution", "testUniversalRouterExactErrors",
    "testUniversalRouterDiagnostic",
}
UR = "0xd97576d5fed11c7c31095d4fd0af612dc736679b"
UR_SHA256 = "949ac3536c8fe99912ade4f65ccea1161230186f0dba83bede70d263083554d3"


def clean(text, endpoint=""):
    if endpoint:
        text = text.replace(endpoint, "<RPC>")
    text = re.sub(r"https?://[^\s\"'<>]+", "<URL>", text)
    return re.sub(r"(?i)[a-z]:[\\/]+Users[\\/]+[^\\/\s]+|/(?:home|Users)/[^/\s]+", "<HOME>", text)


def rpc(url, method, params):
    payload = json.dumps(dict(jsonrpc="2.0", id=1, method=method, params=params)).encode()
    req = urllib.request.Request(url, data=payload, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=20) as response:
        value = json.load(response)
    if "error" in value:
        raise RuntimeError("RPC failed for " + method)
    return value["result"]


def validate_output(text, exit_code):
    """Reject empty runs, wrong names, skipped tests and a missing fuzz count."""
    passes = re.findall(r"\[PASS\]\s+(test\w+)\(", text)
    failures = re.findall(r"\[FAIL[^\]]*\]\s+(test\w+)\(", text)
    counts = re.findall(r"Suite result:.*?(\d+) passed; (\d+) failed; (\d+) skipped", text)
    valid = (
        exit_code == 0 and len(passes) == 9 and set(passes) == NAMES
        and not failures and counts == [("9", "0", "0")]
        and bool(re.search(r"testFuzzMaterialQuotesAllPairs\([^\n]*runs:\s*1024\b", text))
    )
    return {"validated_pass": valid, "pass_names": passes, "failure_names": sorted(set(failures)),
            "reported_counts": counts, "substantive_functions": 8, "logging_only_functions": 1}


def invoke(args, **kwargs):
    return subprocess.run(args, capture_output=True, timeout=180, **kwargs)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--evm", choices=["cancun", "london"], default="cancun")
    parser.add_argument("--port", type=int, default=8562)
    args = parser.parse_args()
    output = Path(tempfile.mkdtemp(prefix="mezo-supplemental-"))
    endpoint = os.environ.get("MEZO_RPC_URL", "")
    record = {"captured_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
              "status": "BLOCKED", "evm": args.evm, "block": BLOCK, "chain_id": CHAIN,
              "public_transactions_sent": 0, "source_sha256": {},
              "limits": ["fixed-state quotes", "no funded swap positive control",
                         "compatible-opcode local EVM, not full Mezo native-precompile equivalence"]}
    node = None
    try:
        if not endpoint:
            raise RuntimeError("MEZO_RPC_URL required; no tests executed")
        forge = os.environ.get("FORGE_BIN") or shutil.which("forge")
        anvil = os.environ.get("ANVIL_BIN") or shutil.which("anvil")
        if not forge or not anvil:
            raise RuntimeError("Already-installed Forge and Anvil are required")
        env = os.environ.copy()
        env["FOUNDRY_PROFILE"] = "default" if args.evm == "cancun" else "london_diagnostic"
        env["NO_COLOR"] = "1"
        for name, tool in [("forge", forge), ("anvil", anvil)]:
            v = invoke([tool, "--version"])
            text = (v.stdout + v.stderr).decode("utf-8", errors="replace")
            if v.returncode or not re.search(r"\b1\.7\.1\b", text) or COMMIT not in text:
                raise RuntimeError(name + " must match Foundry 1.7.1 and the recorded commit")
            record[name + "_version"] = clean(text, endpoint)
        conf = invoke([forge, "config", "--json"], cwd=ROOT, env=env)
        if conf.returncode:
            raise RuntimeError("Effective configuration unavailable")
        settings = json.loads(conf.stdout)
        expected = {"solc": "0.8.24", "evm_version": args.evm, "ffi": False,
                    "test": "test", "src": "src", "optimizer": True, "optimizer_runs": 200}
        if any(settings.get(k) != v for k, v in expected.items()):
            raise RuntimeError("Effective configuration mismatch")
        record["effective_config"] = expected
        record["anvil_backend_hardfork"] = "cancun"
        record["execution_note"] = "Forge executes the selected EVM profile; Anvil is a fixed Cancun state backend. Changing compiler EVM also changes harness bytecode."
        for p in sorted(ROOT.rglob("*")):
            if p.is_file() and p.suffix in {".sol", ".toml"} and not any(x in {"out", "cache"} for x in p.parts):
                record["source_sha256"][p.relative_to(ROOT).as_posix()] = hashlib.sha256(p.read_bytes()).hexdigest()
        record["runner_sha256"] = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
        if int(rpc(endpoint, "eth_chainId", []), 16) != CHAIN:
            raise RuntimeError("Wrong chain")
        header = rpc(endpoint, "eth_getBlockByNumber", [hex(BLOCK), False])
        if not header or header["hash"] != BLOCK_HASH or int(header["number"], 16) != BLOCK:
            raise RuntimeError("Wrong historical block/hash")
        record["block_hash"] = header["hash"]
        record["timestamp"] = int(header["timestamp"], 16)
        code = rpc(endpoint, "eth_getCode", [UR, hex(BLOCK)])
        if hashlib.sha256(bytes.fromhex(code[2:])).hexdigest() != UR_SHA256:
            raise RuntimeError("UniversalRouter runtime mismatch")
        record["universal_router_runtime_sha256"] = UR_SHA256
        with socket.socket() as probe:
            if probe.connect_ex(("127.0.0.1", args.port)) == 0:
                raise RuntimeError("Local port occupied; no existing service will be stopped")
        local = "http://127.0.0.1:" + str(args.port)
        node = subprocess.Popen([anvil, "--fork-url", endpoint, "--fork-block-number", str(BLOCK),
                                 "--hardfork", "cancun", "--host", "127.0.0.1", "--port", str(args.port), "--accounts", "0", "--quiet"],
                                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        ready = False
        for _ in range(60):
            if node.poll() is not None:
                raise RuntimeError("Local Anvil stopped")
            try:
                h = rpc(local, "eth_getBlockByNumber", [hex(BLOCK), False])
                ready = h and h["hash"] == BLOCK_HASH and int(rpc(local, "eth_chainId", []), 16) == CHAIN
                if ready:
                    break
            except Exception:
                pass
            time.sleep(0.25)
        if not ready:
            raise RuntimeError("Local fork unavailable")
        env["MEZO_RPC_URL"] = local
        cmd = [forge, "test", "--match-contract", "^EvidenceControls$", "--fuzz-runs", "1024",
               "--fuzz-seed", SEED, "-vvvv"]
        record["status"] = "FAILED"
        result = invoke(cmd, cwd=ROOT, env=env)
        text = (result.stdout + result.stderr).decode("utf-8", errors="replace")
        (output / "forge.txt").write_text(clean(text, endpoint), encoding="utf-8")
        record["command"] = ["forge", *cmd[1:]]
        record["profile"] = env["FOUNDRY_PROFILE"]
        record["exit_code"] = result.returncode
        record["validation"] = validate_output(text, result.returncode)
        if record["validation"]["validated_pass"]:
            record["status"] = "PASS"
    except Exception as exc:
        record["error"] = clean(str(exc), endpoint)
    finally:
        if node is not None:
            node.terminate()
            try:
                node.wait(timeout=10)
            except subprocess.TimeoutExpired:
                node.kill()
                node.wait(timeout=10)
        (output / "RUN.json").write_text(json.dumps(record, indent=2) + "\n", encoding="utf-8")
    print(record["status"] + ": " + str(output))
    if record.get("error"):
        print(record["error"])
    return 0 if record["status"] == "PASS" else (2 if record["status"] == "BLOCKED" else 1)


if __name__ == "__main__":
    raise SystemExit(main())
