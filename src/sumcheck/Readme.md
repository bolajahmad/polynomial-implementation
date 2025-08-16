### Sumcheck protocol

The sumcheck protocol is a verification process that can allow a verifier, V to verify, 
by doing a significantly less expensive and much faster computation. The Verifier defers the would-be very expensive computation to the other participant, a prover P, who does the computation and tells V the result.

Note that V does not trust P, and wants to confirm the result and know that the prover is not making it up.
Prover must prove that he knows a Univariate Polynomial, when executed as all combinations of the boolean hypercube