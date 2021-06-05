# Digestify

digest·​i·​fy -| _ˈdī-ˌjest-ə-ˌfī

__verb__ - Portmanteau of _digest_ and _verify_. Compute a digest of a file and verify that the digest matches a provided value.

__noun__ - Command line tool to make verification of digests/checksums/hashes on downloaded files a little easier.  Just specity the file and the hash listed on the download host.  It will infer the possible digest algorithm(s) from the length.   It's process exit code will be indicate whether the digest matches.


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
digestify 0.2.1
Verify a file against a digest/checksum/hash.

USAGE:
    digestify <FILE> <DIGEST>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>     File to verify
    <DIGEST>   Digest to compare to.  Must be specified as
               a hexadecimal string.  Case does not matter.
```

If a match is found against one of the digest algorithms, the command will succeed.  Otherwise, it will fail.

## Example Usages

```
# digestify Cargo.lock ac4a0ad1af6773ce6980cc9958cfd18820275f28b8055b93e4d69688e2fd72c8

Verifying 'Cargo.lock' against provided digest of size 64 hex chars / 256 bits.  Candidate digest(s): SHA-256.

 SHA-256: MATCHES
    Expected=ac4a0ad1af6773ce6980cc9958cfd18820275f28b8055b93e4d69688e2fd72c8
      Actual=ac4a0ad1af6773ce6980cc9958cfd18820275f28b8055b93e4d69688e2fd72c8

# echo $?
0
```

```
# digestify Cargo.toml f26c2e4e001349b737a3a5cc5fc4afc87b423773
Verifying 'Cargo.toml' against provided digest of size 40 hex chars / 160 bits.  Candidate digest(s): SHA-1.

 SHA-1: FAIL
        Expected=f26c2e4e001349b737a3a5cc5fc4afc87b423773
          Actual=a30d387b67f1e4d26fd59b3ddb94cfde58d23287

# echo $?
254
```

