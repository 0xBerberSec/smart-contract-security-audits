# 0xBerberSec — Smart Contract Security Research

**Independent Smart Contract Security Researcher**

**Injective Peggy Bridge — 1 Medium finding validated by Code4rena · 21st place in the competition.**
[Official profile](https://code4rena.com/@0xBerberSec) · [Competition results](https://code4rena.com/audits/2026-02-injective-peggy-bridge) · [Verified evidence record](validated-findings/injective-peggy-bridge/README.md)

I investigate DeFi accounting, authorization boundaries and state transitions, using executable checks and economic reasoning to assess whether an observation is a vulnerability.

## Start here

- [Validated findings](validated-findings/README.md) — Injective Peggy Bridge and the official Code4rena record.
- [GMX research and executable harness](protocol-investigations/gmx-reproduction/README.md) — Hardhat tests, Rust model experiments and reproduction commands.
- [Mezo research and current evidence](protocol-investigations/mezo-reproduction/supplemental/README.md) — Foundry controls on the historical fork.
- [Methodology](methodology/README.md) — value-flow reconstruction, invariants, targeted tests and counter-tests.
- [Evidence register](docs/EVIDENCE.md) — claim classes and execution records.
- [Research tools and testing](research-tools-testing/README.md) — tool-assisted source inspection and verification.

**Mezo — current evidence (1 November 2026):** Foundry tests on a historical local fork recorded 60 checked quotes (48 positive), 1,024 fixed-state fuzz cases with six checked quotes per case, and exact Router errors. Old UniversalRouter London PASS labels came from `NotActivated`, not the intended checks; the dated correction preserves the initial experiments. No confirmed exploitable economic vulnerability is claimed.

## Verified results

The public work is grouped by evidence type so the executable material is visible before the methodology.

| Material | Evidence and disclosure status |
| --- | --- |
| [Injective Peggy Bridge](validated-findings/injective-peggy-bridge/README.md) | Official competition outcome; technical finding body and PoC not supplied |
| [GMX — Cross-Architecture DeFi Security Research](protocol-investigations/gmx-cross-architecture-security-research.md) | Independent EVM and Solana research with [reproducible harness code](protocol-investigations/gmx-reproduction/README.md); no externally confirmed GMX finding claimed |
| [Mezo independent research](protocol-investigations/mezo-independent-research.md) | Real-protocol fork investigations: [Pool reserve/quote checks](protocol-investigations/mezo-pool-fuzz.md) and [Router execution boundaries](protocol-investigations/mezo-universal-router.md); bounded results, no exploitable vulnerability claimed |
| [Refund Escrow lab](educational-demos/refund-escrow/README.md) | Educational artifact with local tests, counterexample and fixed-version evidence; no third-party finding credit |

Start with the [GMX harness](protocol-investigations/gmx-reproduction/README.md) for Hardhat, Rust and clean-checkout reproduction commands, or the [Refund Escrow code and tests](educational-demos/refund-escrow/README.md) for a self-contained educational example. Historical executions, local reproductions, model results and external validation remain separate evidence classes; [the evidence policy](docs/SEVERITY_AND_CLAIMS.md) defines them.

## My audit approach

**Value Flow → Security Invariants → Targeted Tests → Counter-Tests → Economic Impact Qualification**

I begin with the system’s assets, actors and dependencies. I reconstruct the accounting and state transitions, select a concrete hypothesis, and test the relevant property with explicit setup and assertions. I then test competing explanations: expected behavior, privileged actions, configuration assumptions, prior findings and the actual source of any apparent gain.
The selected [GMX cross-architecture research case study](protocol-investigations/gmx-cross-architecture-security-research.md) applies this method across EVM and Solana research while preserving explicit evidence limits.

Audit-OS keeps the protocol maps, hypotheses, test artifacts, traces and conclusions connected. Scripts and Codex assist research and test preparation. Their output becomes evidence only through source inspection and appropriate technical checks. [Detailed methodology](methodology/README.md).

## Reproduce the method

[**Refund Escrow — executable educational investigation**](educational-demos/refund-escrow/README.md): original teaching source, a solvency invariant, a bounded callback experiment, economic controls, a checks-effects-interactions fix and regression tests. The captured run has **13 passing tests**, including two 256-case fuzz tests and a fixed-version invariant over **8,192 handler calls**. A separate safety-property test deliberately fails on the vulnerable version, with the actual trace retained.

This example was prepared with Codex assistance on 30 October 2026. It demonstrates a known vulnerability class in a new synthetic protocol; it is not a newly discovered third-party vulnerability or an externally validated finding. No RPC or wallet is required. [Code](educational-demos/refund-escrow/src/RefundEscrow.sol) · [Tests](educational-demos/refund-escrow/test/RefundEscrow.t.sol) · [Results and limits](educational-demos/refund-escrow/EVIDENCE.md).

```sh
cd educational-demos/refund-escrow
python scripts/reproduce.py
```

Requires Foundry on PATH and Python 3.10+. The runner checks the passing suite and the specific intentional safety-property failure, then writes a new local evidence folder.

## Additional research overviews

| Material | Scope |
| --- | --- |
| [Accounting models and measured value](protocol-investigations/accounting-models.md) | Generic method overview drawn from research; underlying technical study withheld |
| [Reward lifecycle and state transitions](protocol-investigations/reward-lifecycle.md) | Generic method overview; no protocol-specific reproduction published |
| [Time-dependent execution boundaries](protocol-investigations/temporal-state.md) | Generic method overview; local experiments do not establish live impact |
| [Economic controls and hypothesis rejection](protocol-investigations/economic-counter-tests.md) | Generic method overview; no private exploit details or test results disclosed |

The four overviews explain reasoning, rather than serve as public technical case reports. The runnable lab is explicitly educational; private third-party protocol PoCs remain withheld. Local test success, historical observations and external validation are separate evidence classes; [the evidence policy](docs/SEVERITY_AND_CLAIMS.md) defines them. LayerZero and Jupiter are not claimed as individually platform-validated findings.

## Portfolio sections

- [Validated Findings](validated-findings/README.md)
- [Independent Security Research](independent-security-research/README.md)
- [Reproducible PoCs and educational demonstrations](reproducible-pocs/README.md)
- [Protocol Investigation Case Studies](protocol-investigations/README.md)
- [Audit Methodology](methodology/README.md)
- [Research Tools & Testing](research-tools-testing/README.md)
- [Code4rena Track Record](code4rena-track-record/README.md)

### Mezo — Independent Protocol Security Research

The [Pool study](protocol-investigations/mezo-pool-fuzz.md) and [UniversalRouter study](protocol-investigations/mezo-universal-router.md) explain the research and its limits. The [historical harness](protocol-investigations/mezo-reproduction/README.md) preserves the initial experiments; use the current evidence linked in **Start here** for the complementary controls.

## Collaboration

Available for **paid, scoped private audit collaborations** and technical evaluations, particularly DeFi accounting and state-transition reviews. A useful engagement starts with explicit scope, source revisions, access conditions and expected deliverables. No previous private-client mandate is claimed by this portfolio.

Contact: [GitHub](https://github.com/0xBerberSec) · [Code4rena](https://code4rena.com/@0xBerberSec)

[Disclosure policy](docs/DISCLOSURE.md) · [Evidence register](docs/EVIDENCE.md) · [Repository navigation](docs/INDEX.md)
