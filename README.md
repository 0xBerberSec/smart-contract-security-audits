# Smart Contract Security Audits

[![Portfolio](https://img.shields.io/badge/portfolio-smart%20contract%20security-blue)](https://github.com/0xBerberSec)
[![Code4rena](https://img.shields.io/badge/Code4rena-0xBerberSec-lightgrey)](https://code4rena.com/@0xBerberSec)

Public portfolio of smart contract security research, audit findings, QA reports, and proof-of-concept work by 0xBerberSec.

This repository focuses on DeFi protocol security, cross-chain systems, lending markets, liquidation flows, invariant analysis, runtime behavior validation, and configuration safety.

## Overview

This is a professional audit portfolio. It separates validated findings from research notes and QA submissions so that every claim is clear, factual, and publicly verifiable.

## Validated Findings

| Protocol | Contest | Severity | Finding | Report |
| --- | --- | --- | --- | --- |
| LayerZero | Stellar Endpoint | Low, validated | ULN302 accepts DVN configurations that are valid under validation but non-executable at runtime, causing deterministic `quote()` / `send()` failure under Soroban budget limits. | [Report](./audits/layerzero-stellar-endpoint/README.md) |

## Security Research

| Protocol | Type | Status | Report |
| --- | --- | --- | --- |
| Jupiter Lend | QA / Low | Submitted on Code4rena | [Report](./audits/jupiter-lend-qa/README.md) |
| Moonwell | Research / PoC | Public research | [Report](./audits/moonwell-comptroller-analysis/README.md) |
| LayerZero Receive Library Rotation | Research note | Pending publication | [Report](./audits/layerzero-receive-library-rotation/README.md) |
| K2 Oracle Cache | Research note | Pending publication | [Report](./audits/k2-oracle-cache/README.md) |
| GMX | Research note | Pending publication | [Report](./audits/gmx-research/README.md) |
| Morpho | Research note | Pending publication | [Report](./audits/morpho-research/README.md) |
| Chainlink | Research note | Pending publication | [Report](./audits/chainlink-research/README.md) |

## Proof of Concepts

| Protocol | PoC | Environment | Link |
| --- | --- | --- | --- |
| LayerZero Stellar Endpoint | Runtime budget exhaustion with combined DVN sets | Rust / Soroban | [PoC](./audits/layerzero-stellar-endpoint/PoC.md) |
| Moonwell | Persistent market entry after failed borrow validation | Foundry / Base fork | [Report](./audits/moonwell-comptroller-analysis/README.md) |

## Featured Work

| Protocol | Type | Severity | Summary |
| --- | --- | --- | --- |
| LayerZero Stellar Endpoint | Validated Code4rena finding | Low, validated | ULN302 validates required and optional DVNs independently while runtime paths process them as a combined set, allowing accepted configurations that fail deterministically under Soroban budget limits. |
| Jupiter Lend | Code4rena QA report | QA / Low | A branch marked as Closed can later be reassigned as `current_branch_id` without lifecycle validation, weakening internal branch state invariants. |
| Moonwell | Research / PoC | Informational | Fork-based analysis showing persistent market membership after failed `borrowAllowed()` validation. |

## Repository Layout

```text
audits/
  layerzero-stellar-endpoint/
    README.md
    PoC.md
  jupiter-lend-qa/
    README.md
  moonwell-comptroller-analysis/
    README.md
    BorrowAllowedDirtyStateFork.t.sol
  layerzero-receive-library-rotation/
    README.md
  k2-oracle-cache/
    README.md
  gmx-research/
    README.md
  morpho-research/
    README.md
  chainlink-research/
    README.md
docs/
  INDEX.md
tools/
scripts/
images/
assets/
```

## Methodology

- Identify protocol invariants and trust boundaries
- Trace state transitions across privileged and user-facing flows
- Compare validation logic with runtime execution behavior
- Build minimal PoCs when reproducibility matters
- Separate confirmed impact from defensive QA observations

## Skills

- Smart contract auditing
- Solidity and EVM review
- Rust and Soroban review
- Cross-chain messaging analysis
- DeFi lending and liquidation analysis
- Invariant and state-machine analysis
- Proof-of-concept development
- Fork testing and unit testing

## Tech Stack

- Solidity
- Rust
- Soroban
- Foundry
- Hardhat
- GitHub
- Code4rena

## License

Reports are published for educational and portfolio purposes. See [LICENSE](./LICENSE).

## Contributing

This is a personal security research portfolio. External corrections or suggestions can be opened as issues or pull requests when they improve accuracy, reproducibility, or clarity.

## Links

- GitHub profile: [0xBerberSec](https://github.com/0xBerberSec)
- Code4rena profile: [0xBerberSec](https://code4rena.com/@0xBerberSec)
- Navigation index: [docs/INDEX.md](./docs/INDEX.md)
