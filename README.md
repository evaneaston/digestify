# Digestify

digest·​i·​fy -| _ˈdī-ˌjest-ə-ˌfī

__noun__ - A command line tool to make verification of digests/checksums/hashes on downloaded files a little easier.  Just specity the file and the hash listed on the download host.  It will infer the possible digest algorithm(s) from the length.   It's process exit code will be indicate whether the digest matches.

__verb__ - Portmanteau of _digest_ and _verify_. Compute a digest of a file and verify that the digest matches a provided value.


## Supported Digests

 <br/>Algorithm                                                          | Bit<br/>Length | Hexadecimal<br/>Length |
 :----------------------------------------------------------------: | :--------: | :----------------: |
 [SHA-512](https://en.wikipedia.org/wiki/SHA-2)                     |  512       |  128
 [SHA-256](https://en.wikipedia.org/wiki/SHA-2)                     |  256       |   64
 [SHA-1](https://en.wikipedia.org/wiki/SHA-1)                       |  160       |   40 
 [MD5](https://en.wikipedia.org/wiki/MD5)                           |  128       |   32
 [CRC-32](https://en.wikipedia.org/wiki/Cyclic_redundancy_check)    |   32       |    8


## Usage

```
$ digestify -h
digestify 0.4.0
Verify a file against a provided digest/hash/checksum.

USAGE:
    digestify <file> <digest>

ARGS:
    <file>      File to verify
    <digest>    Digest to compare to, specified as a case-insensitive hexadecimal string

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

If a match is found against one of the digest algorithms, the command will succeed.  Otherwise, it will fail.

## Example Usages

```
# digestify Cargo.lock 19189c1219285b61ce1e95f9e4fdd5354a926075db9cf5dc62cf1801702c67d2

Verifying 'Cargo.lock' against provided digest of size 64 hex chars / 256 bits.  Candidate digest(s): SHA-256.

 SHA-256: PASS
        Expected=19189c1219285b61ce1e95f9e4fdd5354a926075db9cf5dc62cf1801702c67d2
          Actual=19189c1219285b61ce1e95f9e4fdd5354a926075db9cf5dc62cf1801702c67d2

PASS: Provided digest matches the content.
# echo $?
0
```

```
# digestify Cargo.toml f26c2e4e001349b737a3a5cc5fc4afc87b423773

Verifying 'Cargo.toml' against provided digest of size 40 hex chars / 160 bits.  Candidate digest(s): SHA-1.

 SHA-1: FAIL
        Expected=f26c2e4e001349b737a3a5cc5fc4afc87b423773
          Actual=95ee03e733a0111ac9d6b2073f4acec0f120fae5

FAIL: Provided digest doesn't match any of the candidate digest results.
# echo $?
2
```

