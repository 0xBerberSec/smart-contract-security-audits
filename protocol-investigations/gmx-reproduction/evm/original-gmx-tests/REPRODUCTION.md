# Reproduction from a GMX checkout

The public TypeScript files are extracted at the original `test/guardian/`
location and keep the original relative imports. They are not a standalone
Hardhat project; they require a licensed GMX Synthetics checkout at revision
`10cfbce6144ade8e9d5cd8c811573955bcd1c132` and its dependencies/artifacts.

After placing the two files under that checkout's `test/guardian/` directory:

```text
npx hardhat test --no-compile test/guardian/external-handler-safe-controls.spec.ts
npx hardhat test --no-compile test/guardian/vault-accounting-observations.spec.ts
```

Expected results for this snapshot:

```text
2 passing, exit 0
2 passing, exit 0
```

`--no-compile` is intentional for this evidence snapshot. The original clean
build used Solidity 0.8.29 and produced the artifacts identified in the private
hash manifest. A public consumer must either reproduce that build with the
exact dependency lockfile or supply equivalent verified artifacts; a successful
test command alone does not prove artifact provenance.
