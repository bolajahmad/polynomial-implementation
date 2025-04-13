### Polynomials

Polynomials are are a key cornerstone of Mathematical operations. A polynomial can encode data evem much larger than a usual integer could. Think of an integer such as a BigUint. This can only be as large as itself but a Polynomial can be more. It looks like the following 

```
    f(x) = a<sub>n</sub>x<sup>n</sup>  + .. + a<sub>0</sub
```

n is referred to as the order of the Polynomial. E.g Any line can be represented on a <string>Linear Polynomial</strong> ```f(x) = mx + c```, and something like the ECC can be represented by the <strong>Cubic Polynomial</strong> ```f(x) = x<sup></sup>3 + ax + b```, In cryptography you can use the Polynomial to perform a variety of actions such as store a secret information, generating and verifying proofs of computation among others. 

Consider the Shamir Secret Sharing Scheme (SSSS), allows a group of participants to hold seperate portions of a Key and only when a threshold t, is reached can the original Key be recreated. This Uses the Polynomial to be useful in this scheme

## Univariate Polynomial

The Polynomials we have been looking into have been Univariate Polynomials, the Polynomial always depends on one variable. This is very basic because it has a 2-D dependency. 

## Univariate Implementation


## Multilinear Polynomial(s)

A multilinear polynomial is a polynomial `f(x<sub>1</sub>,..,x<sub>n</sub>)`, where every variable is of a degree of maximum 1 (e.g. 2ab + 3bc. has 3 variables with degrees (1,1,0) & (0,1,1)).
