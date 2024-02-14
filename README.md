# Overview

Rust project made to be able to encrypt and decrypt files, along with polynomial library included to allow public key encryption.
In progress.

# Current Problems:
Not actually secure (can be fixed by changing polynomial based on itself every time it's used, but I'm gonna write a program that cracks the encryption before I fix that)

Cannot be used for public-key encryption. It must:\
    - Be able to find a prime polynomial\
    - Factor a semiprime polynomial based on having only one factor\
    - (I'm not using an integral domain, which will make this harder oops)\
