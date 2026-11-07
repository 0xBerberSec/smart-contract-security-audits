# EVM sanitized derivation

The contract and tests here are an independent educational derivation of the
accounting questions examined in the private ExternalHandler work. They are
not the GMX contracts, not a copy of the original test, and not evidence of a
production vulnerability.

The three assertions make the intended proof boundary explicit:

1. an explicitly listed balance is refunded;
2. an omitted balance remains in the ledger;
3. a later revert rolls back the route state.
