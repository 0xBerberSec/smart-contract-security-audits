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
| What technical material can I inspect here? | [Evidence register](docs/EVIDENCE.md) and [reproduction status](reproducible-pocs/README.md) |
| How do I evaluate difficult hypotheses? | [Investigation overviews](protocol-investigations/README.md) |
| Which tools and techniques do I use? | [Research tools and testing](research-tools-testing/README.md) |

## My audit approach

**Value Flow → Security Invariants → Targeted Tests → Counter-Tests → Economic Impact Qualification**

I begin with the system’s assets, actors and dependencies. I reconstruct the accounting and state transitions, select a concrete hypothesis, and test the relevant property with explicit setup and assertions. I then test competing explanations: expected behavior, privileged actions, configuration assumptions, prior findings and the actual source of any apparent gain.

Audit-OS keeps the protocol maps, hypotheses, test artifacts, traces and conclusions connected. Scripts and Codex assist research and test preparation. Their output becomes evidence only through source inspection and appropriate technical checks. [Detailed methodology](methodology/README.md).

## Selected work

| Material | Evidence and disclosure status |
| --- | --- |
| [Injective Peggy Bridge](validated-findings/injective-peggy-bridge/README.md) | Official competition outcome; technical finding body and PoC not supplied |
| [Accounting models and measured value](protocol-investigations/accounting-models.md) | Generic method overview drawn from research; underlying technical study withheld |
| [Reward lifecycle and state transitions](protocol-investigations/reward-lifecycle.md) | Generic method overview; no protocol-specific reproduction published |
| [Time-dependent execution boundaries](protocol-investigations/temporal-state.md) | Generic method overview; local experiments do not establish live impact |
| [Economic controls and hypothesis rejection](protocol-investigations/economic-counter-tests.md) | Generic method overview; no private exploit details or test results disclosed |

The four overviews explain reasoning, rather than serve as public technical case reports. This release contains no runnable protocol PoC. Local test success, historical observations and external validation are separate evidence classes; [the evidence policy](docs/SEVERITY_AND_CLAIMS.md) defines them. LayerZero and Jupiter are not claimed as individually platform-validated findings.

## Portfolio sections

- [Validated Findings](validated-findings/README.md)
- [Independent Security Research](independent-security-research/README.md)
- [Reproducible PoCs](reproducible-pocs/README.md)
- [Protocol Investigation Case Studies](protocol-investigations/README.md)
- [Audit Methodology](methodology/README.md)
- [Research Tools & Testing](research-tools-testing/README.md)
- [Code4rena Track Record](code4rena-track-record/README.md)

## Collaboration

Available for **paid, scoped private audit collaborations** and technical evaluations, particularly DeFi accounting and state-transition reviews. A useful engagement starts with explicit scope, source revisions, access conditions and expected deliverables. No previous private-client mandate is claimed by this portfolio.

Contact: [GitHub](https://github.com/0xBerberSec) · [Code4rena](https://code4rena.com/@0xBerberSec)

[Disclosure policy](docs/DISCLOSURE.md) · [Evidence register](docs/EVIDENCE.md) · [Repository navigation](docs/INDEX.md)
