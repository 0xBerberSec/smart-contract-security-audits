# Smart Contract Security Audits

Smart contract security portfolio by 0xBerberSec.

Focused on DeFi, cross-chain security, invariant analysis, and proof-of-concept driven vulnerability research.

**Current portfolio:** 1 validated Code4rena Low finding + public QA and independent security research.

## Portfolio Snapshot

| Category | Count | Scope |
| --- | ---: | --- |
| Validated findings | 1 | Code4rena validated Low |
| QA research | 1 | Public Code4rena QA report |
| Independent security research | 1 | Public PoC-driven research |

## Validated Findings

| Protocol | Platform | Severity | Status | Report |
| --- | --- | --- | --- | --- |
| LayerZero Stellar Endpoint | Code4rena | Low | Validated | [Report](./audits/layerzero-stellar-endpoint/README.md) |

## Independent Security Research

| Protocol | Type | Status | Report |
| --- | --- | --- | --- |
| Jupiter Lend | Code4rena QA Report | Public QA Research | [Report](./audits/jupiter-lend-qa/README.md) |
| GMX Synthetics | Independent Security Research | Public independent research | [Report](./audits/gmx-disabled-integration-relay-bypass/README.md) |

## Repository Structure

```text
audits/
  layerzero-stellar-endpoint/
    README.md
    PoC.md
  jupiter-lend-qa/
    README.md
  gmx-disabled-integration-relay-bypass/
    README.md
    PoC.md
docs/
  INDEX.md
  METHODOLOGY.md
  POC_STANDARD.md
  SEVERITY_AND_CLAIMS.md
```

## Research Standards

- [Methodology](./docs/METHODOLOGY.md)
- [PoC Standard](./docs/POC_STANDARD.md)
- [Severity and Claims](./docs/SEVERITY_AND_CLAIMS.md)

## Skills

- Solidity / EVM review
- Rust / Soroban review
- DeFi lending analysis
- Cross-chain messaging analysis
- Invariant and state-machine analysis
- Proof-of-concept development
- Foundry and fork-based testing

## Contact

- GitHub: [0xBerberSec](https://github.com/0xBerberSec)
- Code4rena: [0xBerberSec](https://code4rena.com/@0xBerberSec)
- Index: [docs/INDEX.md](./docs/INDEX.md)
