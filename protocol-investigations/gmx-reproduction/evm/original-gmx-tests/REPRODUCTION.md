# Reproduction from a clean GMX checkout

The supported entry point is:

```text
python protocol-investigations/gmx-reproduction/scripts/reproduce-original-evm.py
```

It clones `https://github.com/gmx-io/gmx-synthetics.git` into an isolated
directory, checks out `10cfbce6144ade8e9d5cd8c811573955bcd1c132`, installs from
the official lockfile when present, compiles with Hardhat, copies the two public
adaptations and executes both files. It exits non-zero on clone, dependency,
compilation, assertion or test-count failure. It never uses Audit-OS, a
production RPC or a wallet. Use `--workdir <path>` to preserve logs.

This command is an executable verification mechanism, not a presumed result:
PASS is reported only after all stages actually complete.

The public TypeScript files are extracted at the original `test/guardian/`
location and keep the original relative imports. They are not a standalone
Hardhat project; they require a licensed GMX Synthetics checkout at revision
`10cfbce6144ade8e9d5cd8c811573955bcd1c132` and its dependencies/artifacts.

Manual execution (equivalent to the runner) after placing the two files under
that checkout's `test/guardian/` directory:

```text
npx hardhat test --no-compile test/guardian/external-handler-safe-controls.spec.ts
npx hardhat test --no-compile test/guardian/vault-accounting-observations.spec.ts
```

Expected results for this snapshot:

```text
2 passing, exit 0
2 passing, exit 0
```

The historical four-test result used `--no-compile` against previously verified
artifacts. It remains historical evidence and is not substituted for the clean
runner's result. If clean compilation or either test fails, the public result is
`BLOCKED`/`FAILED`, not PASS.
