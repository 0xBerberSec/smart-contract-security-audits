# Source and licence notices

## GMX Solana model

The 39 Rust files under `solana/model/src/` are copied from the isolated
`gmsol-model` crate at source revision `3f10e9cd14bd23765567503c2a20e758a3fcbf39`.
They are source-preserving research artefacts, not newly authored protocol
code. The accompanying `solana/model/LICENSE.gmx-solana` is the licence text
from the source workspace and must remain with any redistribution.

The source workspace identifies the licensed work as GMX Solana Rust contracts
and grants non-production use under its stated terms. This harness is a local,
non-production reproduction and makes no deployment or commercial-use claim.

## Rust dependencies

The harness does not vendor third-party Rust source. Cargo resolves declared
crates (`ruint`, `rust_decimal`, `num-traits`, `num_enum`, `typed-builder`,
`thiserror`, and their transitive dependencies) from crates.io. Their licences
remain governed by their respective package metadata and the generated
lockfile; this repository does not relicense them.

## EVM derivation

`evm/contracts/RefundLedger.sol` and
`evm/test/refund-ledger.spec.ts` are original sanitized educational code under
the repository's existing portfolio licence. They are not copied GMX source.
