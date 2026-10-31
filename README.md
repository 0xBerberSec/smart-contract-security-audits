# 0xBerberSec — Smart Contract Security Research

**Independent Smart Contract Security Researcher**

**Injective Peggy Bridge — 1 Medium finding validated by Code4rena · 21st place in the competition.**
[Official profile](https://code4rena.com/@0xBerberSec) · [Competition results](https://code4rena.com/audits/2026-02-injective-peggy-bridge) · [Verified evidence record](validated-findings/injective-peggy-bridge/README.md)

I investigate DeFi accounting, authorization boundaries and state transitions, using executable checks and economic reasoning to assess whether an observation is a vulnerability.

## Start here

| Question | Read |
| --- | --- |
| What has been independently validated? | [Injective Peggy Bridge — official result](validated-findings/injective-peggy-bridge/README.md) |
| How do I conduct an audit? | [Audit methodology](methodology/README.md) and [research workflow](methodology/workflow.md) |
| What technical material can I inspect here? | [Runnable educational lab](educational-demos/refund-escrow/README.md), [actual results](educational-demos/refund-escrow/EVIDENCE.md) and [evidence register](docs/EVIDENCE.md) |
| How do I evaluate difficult hypotheses? | [Investigation overviews](protocol-investigations/README.md) |
| Which tools and techniques do I use? | [Research tools and testing](research-tools-testing/README.md) |

## My audit approach

**Value Flow → Security Invariants → Targeted Tests → Counter-Tests → Economic Impact Qualification**

I begin with the system’s assets, actors and dependencies. I reconstruct the accounting and state transitions, select a concrete hypothesis, and test the relevant property with explicit setup and assertions. I then test competing explanations: expected behavior, privileged actions, configuration assumptions, prior findings and the actual source of any apparent gain.

Audit-OS keeps the protocol maps, hypotheses, test artifacts, traces and conclusions connected. Scripts and Codex assist research and test preparation. Their output becomes evidence only through source inspection and appropriate technical checks. [Detailed methodology](methodology/README.md).

## Reproduce the method

[**Refund Escrow — executable educational investigation**](educational-demos/refund-escrow/README.md): original teaching source, a solvency invariant, a bounded callback experiment, economic controls, a checks-effects-interactions fix and regression tests. The captured run has **13 passing tests**, including two 256-case fuzz tests and a fixed-version invariant over **8,192 handler calls**. A separate safety-property test deliberately fails on the vulnerable version, with the actual trace retained.

This example was prepared with Codex assistance on 30 October 2026. It demonstrates a known vulnerability class in a new synthetic protocol; it is not a newly discovered third-party vulnerability or an externally validated finding. No RPC or wallet is required. [Code](educational-demos/refund-escrow/src/RefundEscrow.sol) · [Tests](educational-demos/refund-escrow/test/RefundEscrow.t.sol) · [Results and limits](educational-demos/refund-escrow/EVIDENCE.md).

```sh
cd educational-demos/refund-escrow
python scripts/reproduce.py
```

Requires Foundry on PATH and Python 3.10+. The runner checks the passing suite and the specific intentional safety-property failure, then writes a new local evidence folder.

## Selected work

| Material | Evidence and disclosure status |
| --- | --- |
| [Injective Peggy Bridge](validated-findings/injective-peggy-bridge/README.md) | Official competition outcome; technical finding body and PoC not supplied |
| [Refund Escrow lab](educational-demos/refund-escrow/README.md) | New educational artifact with actual local tests, counterexample and fixed-version evidence; no third-party finding credit |
| [Mezo independent research](protocol-investigations/mezo-independent-research.md) | Real-protocol fork investigations: [Pool reserve/quote checks](protocol-investigations/mezo-pool-fuzz.md) and [Router execution boundaries](protocol-investigations/mezo-universal-router.md); bounded results, no exploitable vulnerability claimed |
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

Selected private-to-public-safe summaries cover Pool bounded fuzzing and UniversalRouter permission boundaries. These studies distinguish historical transcripts from fresh reproductions and do not claim a confirmed Mezo vulnerability or client audit. [Read the Mezo research overview](protocol-investigations/mezo-independent-research.md).

## Collaboration

Available for **paid, scoped private audit collaborations** and technical evaluations, particularly DeFi accounting and state-transition reviews. A useful engagement starts with explicit scope, source revisions, access conditions and expected deliverables. No previous private-client mandate is claimed by this portfolio.

Contact: [GitHub](https://github.com/0xBerberSec) · [Code4rena](https://code4rena.com/@0xBerberSec)

[Disclosure policy](docs/DISCLOSURE.md) · [Evidence register](docs/EVIDENCE.md) · [Repository navigation](docs/INDEX.md)
