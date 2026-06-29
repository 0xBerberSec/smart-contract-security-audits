# PoC Standard

[Back to index](./INDEX.md)

Every public PoC should make the claim easier to verify, not harder to trust.

## Required Elements

A PoC should include:

- protocol and component under test;
- test file or patch location;
- exact command used to run the test;
- expected result;
- observed result;
- what the PoC proves;
- what the PoC does not prove.

## Preferred Structure

```text
Overview
Environment
Steps
Expected Result
Observed Result
Conclusion
```

## Quality Rules

- The PoC should demonstrate the exact written claim.
- The setup should avoid unnecessary mocks unless the report explains why they are acceptable.
- The exploit path should begin from a realistic external entry point whenever possible.
- Assertions should check final balances, stored state, or reverted behavior directly.
- The test name should describe the security property being violated.

## Conservative Framing

Passing tests do not automatically prove production severity. A PoC can prove a primitive while the report remains QA, Low, or independent research depending on the assumptions and impact.

Public write-ups should separate:

- demonstrated behavior;
- inferred risk;
- unproven broader impact.

[Back to index](./INDEX.md)
