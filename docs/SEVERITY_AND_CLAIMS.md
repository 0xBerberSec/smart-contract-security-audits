# Evidence Classes and Claim Discipline

| Class | Required basis | Appropriate claim |
| --- | --- | --- |
| Educational demonstration | Original synthetic protocol and actual local execution | Explain the known vulnerability class and test scope; no third-party discovery or contest credit |
| Officially validated finding | Attributable external ruling or official result | State the exact accepted outcome and source |
| Independent finding with local evidence | Identifiable test, setup and result supporting a specific behavior | State the demonstrated behavior and prerequisites; no external endorsement |
| Historical documented result | A saved execution record or a clearly identified historical report | Label reported vs raw evidence; do not imply a fresh rerun |
| Observed security behavior | A measured or documented anomaly | Explain missing exploitability or impact |
| Unverified hypothesis | A plausible idea without sufficient demonstration | Present as unresolved; no confirmed vulnerability claim |
| Refuted / expected / duplicate | Concrete rebuttal, control, design explanation or duplication basis | Record what was tested and why the vulnerability interpretation was rejected |
| Source-limited / blocked research | Missing source, dependency, fixture, RPC or access | State which property could not be verified |

These labels describe evidence, not a severity ladder. A result can be locally demonstrated, historically reported, unverified externally and withheld from disclosure at the same time.

A passing exploit-style test is not automatically a realistic exploitation path. A checkout HEAD is not automatically the audited revision. A reverted or skipped scenario is not successful-path coverage. No bounded negative result establishes general protocol safety.

An absent submission is **unknown**, not proven never submitted. Prior publication by a researcher is not sponsor consent. An aggregate report is not proof of individual credit. Local severity estimates are not official ratings.

[Validation process](../methodology/validation.md) · [Evidence register](EVIDENCE.md) · [Portfolio](../README.md)
