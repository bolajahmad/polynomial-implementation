## Shamir Secret Sharing Scheme (SSSS)

Consider a situation where you have a public vault that needs to be protected. The vault is locked with a passcode but any of the 10 Vault managers might need access to it at anytime.
How do we secure this vault when there's only one passcode? It would be vulnerable to give every manager the passcode directly because the they would be able to access the vault without involving anyone else.

The approach of SSSS is to divide the secret into different parts and assign each member with a part. Once threshold number of parts are recovered by a manager, acces to the vault is granted.

### Technique

Once the passcode is derived, it needs to be used as part of a polynomial (in any way). Some approaches involve using the passcode as the coefficient of the x^0 term (the secret can be gotten by evaluating the polynomial at x=0). Some more creative approaches might involve combining the coefficient of multiple terms in the polynomial (secret = coefficient of x^n + coefficient of x^n+1 etc).  