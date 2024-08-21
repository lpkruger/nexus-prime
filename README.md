## Introduction ##

This exercise implements a simple zero-knowledge authentication scheme based on the difficulty of prime number factorization.  There is a target number which is the product of two large primes, and passing the authentication means passing to the ZKVM the two non-trivial prime factors.  The proof of the ZKVM execution can be used to prove the authetication was successful or not without revealing the private inputs.

The target product of primes could be a public constant, in this exercise it is hardcoded as the constant
79484266487585574680924259491585461266177510114289222263303228151162937255137

Passing the prime factors as a private input required use of the SDK, as I didn't find any support for the private inputs feature in the CLI "cargo nexus" version of the NexusZKVM.



### Limitations ###

The prime numbers used in this exercise are too small to be secure (the U256 type is limited to 256-bit numbers and can be factored in reasonable time), however it has the advantage for development that creating the proof only takes a couple of hours.  Hopefully as the ZKVM is optimized and with sufficient parallelization this can be trivially raised to a secure number of bits.

There are several big number crates for Rust, the one I chose (bnum) is the only one that built without errors in the VM target.  As big numbers are generally useful in crypto algorithms, it would be useful to debug some of the other libraries, such as num-bigint, rug, and ibig.

