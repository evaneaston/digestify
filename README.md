# Digestify

Command line tool to make verification of digests/checksums/hashes on downloaded files a little easier.  Just specity the file and the hash listed on the download host.  It will infer the possible digest algorithm(s) from the length.   

## Supported Digests

* SHA-512
* SHA-256
* MD-5 ‡

‡ Yeah, yeah.  But some sites still use these.

## Example Usage

```
$ digestify Cargo.lock ac4a0ad1af6773ce6980cc9958cfd18820275f28b8055b93e4d69688e2fd72c8

Verifying 'Cargo.lock' against provided digest of size 64 hex chars / 256 bits.  Candidate digest(s): SHA-256.

 SHA-256: MATCHES
    Expected=ac4a0ad1af6773ce6980cc9958cfd18820275f28b8055b93e4d69688e2fd72c8
      Actual=ac4a0ad1af6773ce6980cc9958cfd18820275f28b8055b93e4d69688e2fd72c8
```


## Usage

```
$ digestify -h
digestify 0.1.0
Verify a file against a digest/checksum/hash.

USAGE:
    digestify <FILE> <DIGEST>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>      File to verify
    <DIGEST>    Digest to compare to.  Must be specified as a
                hexadecimal string.  Case does not matter.
```

