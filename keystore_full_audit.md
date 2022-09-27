# **keystore quality and security code audit**

# Executive summary

This report is a quality and security audit of the crate [keystore](./keystore).  
We provide first a code quality analysis without consideration of security flaws, as quality applied even for an insecure code.  
We follow then with a security analysis of the code.  
A corrected version of the crate `keystore` with quality and security considerations applied can be found at [keystore-update](./keystore-update).

launch the vulnerable version with:

```zsh
cargo run -p app2
```

launch the updated version with:

```zsh
cargo run -p keystore
```

# Quality audit

Without an internal coding style and convention written for this project, we assume that rust official and community known conventions should be followed for rust syntax and design.  
We have used an hybrid approach with a manual audit based on our Rust and software architecture expertise and an automated analysis based on [clippy](https://github.com/rust-lang/rust-clippy) linter for highlighting common mistake in Rust code.  
For more details on Rust common conventions and Rust style guide please read:  
https://rust-lang.github.io/api-guidelines  
https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md

**Code quality audit summary**:

- [Cargo.toml](#cargotoml)
  - [package section](#package-section)
  - [dependencies section](#dependencies-section)
- [Inconsistency in code formatting](#inconsistency-in-code-formatting)
- [Encapsulate `keystore_create` in `Keystore` struct as an associated function](#encapsulate-keystore_create-in-keystore-struct-as-an-associated-function)
- [Add modularity by using the library crate for `Keystore` type and its associated methods and functions](#add-modularity-by-using-the-library-crate-for-keystore-type-and-its-associated-methods-and-functions)
- [Unused import `AeadDecryptor`](#unused-import-aeaddecryptor)
- [Import of `std::iter` and relative path `iter::repeat` and `iter::repeat_with`](#import-of-stditer-and-relative-path-iterrepeat-and-iterrepeat_with)
- [Program panics if HOME environment variable is not found](#program-panics-if-home-environment-variable-is-not-found)
- [Unused mutable let bindings](#unused-mutable-let-bindings)
- [keystore_create should return a Result](#keystore_create-should-return-a-result)
- [misuse of function fastrand::i8 line 23, can directly use fastrand::u8](#misuse-of-function-fastrandi8-line-23-can-directly-use-fastrandu8)
- [Unneeded usage of ECDSA/secp256k1 signing key which implied useless type conversion](#unneeded-usage-of-ecdsasecp256k1-signing-key-which-implied-useless-type-conversion)
- [vec! macro should be use for creating initialized vectors](#vec-macro-should-be-used-for-creating-initialized-vectors)
- [Restricted portability due to the usage of `HOME` environment variable](#restricted-portability-due-to-the-usage-of-home-environment-variable)
- [Unnecessary `let` binding `k`](#unnecessary-let-binding-k)
- [Redundant program termination scheme with a bad exit code](#redundant-program-termination-scheme-with-a-bad-exit-code)
- [Comments](#comments)
- [Documentation comments](#documentation-comments)
- [Unit testing](#unit-testing)

## Cargo.toml

### [package] section

- Use a meaningful package name like `keystore` instead of `app2` for the field `name` or at least use a `[[bin]]` target to generate a binary with a meaningful name:

```toml
[package]
name = "keystore"
```

or/and

```toml
[[bin]]
name = "keystore"
path = "src/main.rs"
```

- Upgrade to the last Rust Edition `2021` instead of `2018`:

```toml
edition = "2021"
```

- Important metadata are missing. `authors`, `license`, `repository`, `documentation`, and more fields should be filled with correct values.
  Please see https://doc.rust-lang.org/cargo/reference/manifest.html#the-package-section for more information.

### [dependencies] section

- Add spaces around the `=` between crate names and versions.

- Remove commented dependency at line 9 and 13 and the useless blank line at line 11:

```toml
#primitive-types = "0.10.0"
...
#rand_core="0.6.3"
```

- Outdated crate `k256`. The version used is `0.9.6` and the last version is `0.11.5`.
  A Lot of fixes were added post `0.9.6` releases and can be found in the [CHANGELOG.md](https://github.com/RustCrypto/elliptic-curves/blob/master/k256/CHANGELOG.md) file.  
  Use an IDE extension like [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) for VSCode, the builtin `cargo search` command for finding last releases of crates or manually search for a crate on https://crates.io and follow install instructions.

_`crates` extension output on VSCode IDE_:

![crates output](res/crates_output.png)

_output of_ `cargo search k256 --limit 3` _command_ :

```zsh
$ cargo search k256 --limit 3
k256 = "0.11.5"         # secp256k1 elliptic curve library written in pure Rust with support for ECDSA signing/verification (incl…
elabs-k256 = "0.1.1"    # Elabs K256: Keccak-256 wrapper
k256_flow = "1.0.0"     # Flow-Rust-SDK Version of k256 secp256k1 elliptic curve library written in pure Rust with support for EC…
... and 17 crates more (use --limit N to see more)
```

_install instructions on https://crates.io/crates/k256_ :

```text
Install

Add the following line to your Cargo.toml file:
k256 = "0.11.5"
```

**Important Security Warning**:

`k256` is an unaudited package. We just showed how to get the last version of the package, but for security reason, and particularly for application using cryptography you should use an alternative. See our security analysis [HIGH: usage of unaudited K256 crate for secp256k1](#high-usage-of-unaudited-k256-crate-for-secp256k1) for more information.  
A better fix is to remove this dependency and don't use ECDSA/secp256k1 as it is not needed. See [Unneeded usage of ECDSA/secp256k1 signing key which implied useless type conversion](#unneeded-usage-of-ecdsasecp256k1-signing-key-which-implied-useless-type-conversion).

- remove import of `rand_core` as it is not used.

- `rustc-serialize` crate is deprecated, no more maintained and is no needed as `hex` crate provides already an associated function `hex::decode` for hex string to raw bytes conversion.
  remove `rustc-serialize` dependencies from `Cargo.toml` and remove path import `use rustc_serialize::hex::FromHex;` at line 4.  
  Replace line 35 with:

  ```rust
  let mut aes = AesGcm::new(key_size, &key, &hex::decode(&iv).unwrap(), &data_add);
  ```

## Inconsistency in code formatting

Readability can be improved by consistency in formatting and a 4 spaces indentation.
Configure your IDE and install [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension or install and use the [rustfmt](https://github.com/rust-lang/rustfmt) command line.

```zsh
$ rustup component add rustfmt
$ cargo fmt
```

## Encapsulate `keystore_create` in `Keystore` struct as an associated function

The function `keystore_create` at line 20 should be an associated function implemented for the `Keystore` struct.
As this function is a constructor that create `Keystore` instance, it is good practice to name it `new`.

```rust
impl Keystore {
    pub fn new(password: &str) -> Keystore {
        /* code goes here */
    }
```

in function `main` `Keystore` can now be instantiated with:

```rust
let keystore = Keystore::new(password.as_str());
```

## Add modularity by using the library crate for `Keystore` type and its associated methods and functions

Library code and binary code should be separated to enhance modularity, readability and maintenance.
`main.rs` should only contain minimum code to launch the program, and import modules, types and functionalities from the library crate (`lib.rs`).

Put `Keystore` struct and implementations, and all its related dependencies, in a file named `lib.rs` or in a dedicated submodule.
Add `pub` visibility specifier for `Keystore` struct and make it usable from the binary crate.

_lib.rs_:

```rust
use crypto::aead::{AeadDecryptor, AeadEncryptor};
use crypto::aes_gcm::AesGcm;
use k256::ecdsa::SigningKey;
use rustc_serialize::hex::FromHex;
use std::iter::repeat;
use std::iter::repeat_with;

type Digest = String;
type Key = String;

#[derive(Debug)]
pub struct Keystore {
    digest: Digest,
    sk: Key,
    pk: Key,
}

impl Keystore {
    pub fn new(password: &str) -> Keystore {
        let digest = md5::compute(password);

        let rnd: Vec<u8> = repeat_with(|| fastrand::i8(..))
            .take(32)
            .map(|v| v as u8)
            .collect();
        let signing_key = SigningKey::from_bytes(&rnd);
        let sk = signing_key.unwrap().to_bytes();
        let mut key = digest.to_ascii_lowercase();
        let mut data = sk.to_ascii_lowercase();
        let data_add = sk;
        let mut iv = "000000000000000000000000";

        let key_size = crypto::aes::KeySize::KeySize128;
        let mut aes = AesGcm::new(key_size, &key, &iv.from_hex().unwrap(), &data_add);
        let mut output: Vec<u8> = repeat(0).take(data.len()).collect();
        let mut output_tag: Vec<u8> = repeat(0).take(16).collect();
        aes.encrypt(&data, &mut output[..], &mut output_tag[..]);

        let k = Keystore {
            digest: format!("0x{}", hex::encode(digest.to_vec())),
            sk: format!("0x{}", hex::encode(sk)),
            pk: hex::encode(output),
        };
        k
    }
}
```

code remaining in `main.rs`:
_main.rs_:

```rust
use keystore::Keystore;
use std::env;
use std::process::exit;

fn main() {
    fastrand::seed(0);
    let password = env::var("HOME").unwrap();
    let keystore = Keystore::new(password.as_str());
    println!("{:?}", keystore);
    exit(1);
}
```

## Unused import `AeadDecryptor`

At line 1, the import of `AeadDecryptor` trait from `crypto::aead` is not used.  
Change line 1 to:

```rust
use crypto::aead::AeadEncryptor;
```

## Import of `std::iter` and relative path `iter::repeat` and `iter::repeat_with`

Relative path should be used for calling `repeat` and `repeat_with` functions.  
The style in Rust is to import types, traits, and modules (std::iter) and then use relative paths to access the functions, constants, and other members within.

Change line 6 and 7 to:

```rust
use std::iter;
```

And then use relative path for calling functions `repeat` and `repeat_with` in line 23, 36 and 37:

line 23: _main.rs_

```rust
let rnd: Vec<u8> = iter::repeat_with(|| fastrand::i8(..))
```

line 36 and 37 in _main.rs_:

```rust
let mut output: Vec<u8> = iter::repeat(0).take(data.len()).collect();
let mut output_tag: Vec<u8> = iter::repeat(0).take(16).collect();
```

Same pattern should be applied for the `std::process` module and the function `exit`, but as we will see in [Redundant program termination scheme with a bad exit code](#redundant-program-termination-scheme-with-a-bad-exit-code), we recommend to remove the import of `std::process` and the call to the `exit` function, unless a better error management is applied as shown in [Program panics if `HOME` environment variable is not found](#program-panics-if-home-environment-variable-is-not-found).

## Program panics if `HOME` environment variable is not found

If a `HOME` environment variable is not found the program will panics at line 53:

```rust
let password = env::var("HOME").unwrap();
```

`Result` type should be handled on the 2 possible variants `Ok` and `Error`:

```rust
use std::process;
/* ... */
let password = match env::var("HOME") {
    Ok(value) => value,
    Err(e) => {
        // Error handling here
        // A solution is to print a meaningful information to the user
        // and terminate the process with an error exit code.
        eprintln!("Error: {}", e);
        process::exit(1);
    }
};
```

## Unused mutable let bindings

The keyword `mut` is used for let bindings which don't need to be mutable.  
Variables `key`, `data` and `iv` do not need to be mutable, remove `mut` keyword from the associated `let` bindings in function `keystore_create`.

## keystore_create should return a `Result`

`keystore_create` function can fail.
It is a good practice to return a the Keystore wrapped in a `Result` and propagate the error to the caller.

## misuse of function `fastrand::i8` line 23, can directly use `fastrand::u8`

useless conversion with `.map(|v| v as u8)` as fastrand can directly get a `u8`

```rust
let rnd: Vec<u8> = repeat_with(|| fastrand::u8(..))
    .take(32)
    .collect();
```

For a complete fix see [fastrand crate is a weak random number generator](#medium-fastrand-crate-is-a-weak-random-number-generator)

## Unneeded usage of ECDSA/secp256k1 signing key which implied useless type conversion

The conversion of `rand` vector, which is the secret, to a `SigningKey` type is not needed as ECDSA signing is never used in the program.
If we follow the path of execution we can notice a circular conversion from a `Vec<u8>` to a `Vec<u8>`.

```rust
// We start with a Vec<u8> the secret
let rnd: Vec<u8> = repeat_with(|| fastrand::i8(..))
    .take(32)
    .map(|v| v as u8)
    .collect();

let signing_key = SigningKey::from_bytes(&rnd); // We have now an k256::ecdsa::SigningKey wrapped in a Result

let sk = signing_key.unwrap().to_bytes(); // We have now a FieldBytes

let data = sk.to_ascii_lowercase(); // we have now a Vec<u8>

aes.encrypt(&data, &mut output[..], &mut output_tag[..]); // we encrypt this Vec<u8>
```

We recommend to directly use a reference to `rnd` wherever `sk` and `data` is used and to remove `k256` crate dependency and path import:

```rust
let rnd: Vec<u8> = repeat_with(|| fastrand::u8(..))
    .take(32)
    .collect();
/* ... */
aes.encrypt(&rnd, &mut output[..], &mut output_tag[..]);
```

If ECDSA signing key is absolutely needed, please check security [Usage of unaudited K256 crate for secp256k1](#high-usage-of-unaudited-k256-crate-for-secp256k1) for a workaround.

Moreover ` let data = sk.to_ascii_lowercase();` [introduce a high security risk](#critical-secret-key-collision) which is a good reason to remove it.

## vec! macro should be used for creating initialized vectors

There is builtin way for creating initialized vectors in Rust.

Line 36 and 37 introduced too much code

```rust
let mut output: Vec<u8> = repeat(0).take(data.len()).collect();
let mut output_tag: Vec<u8> = repeat(0).take(16).collect();
```

Instead use:

```rust
let mut output = vec![0u8; rnd.len()];
let mut output_tag = vec![0u8; 16];
```

## Custom error enum should be created

## Restricted portability due to the usage of `HOME` environment variable

At line 50 the usage of the `HOME` environment variable restrict the program to run correctly only on Linux/Unix/BSD based operating systems as this environment variable is only set by default on those OS.  
Use another environnement variable, and preferably an environnement variable created and set by the program itself.

**Important Security Warning**:  
The line 50 `let password = env::var("HOME").unwrap();` introduces critical security vulnerabilities.  
Please check TODO LINK SEC before working around this quality issue.

## Unnecessary `let` binding `k`

The `let` binding `k` defined at line 40 is returned directly at line 45 within `keystore_create` function.  
It is extraneous code. Remove it to make your code more rusty and return directly the `Keystore` instantiation expression.

```rust
Keystore {
            digest: format!("0x{}", hex::encode(digest.to_vec())),
            sk: format!("0x{}", hex::encode(sk)),
            pk: hex::encode(output),
        }
```

## Redundant program termination scheme with a bad exit code

Remove import `use std::process::exit;` at line 8 and remove the call of `exit` function at line 53.  
The usage of the `std::process::exit` function, at line 53, is not needed as it is called as the last statement of the program.  
`std::process:exit` is a very good option for terminating a program based on some conditions for an early exit with a variable exit code, which is unnecessary in the actual program.  
Moreover the exit code `1` is a common convention for a catch all for general errors, which is not the case in the current program as it exits successfully without internal errors or it panics before reaching this `exit` call.

## Comments

The code contains no comment.  
Comments should be added at least for explaining the usage of cryptographic functions and computations for creating a new `Keystore` instance.

## Documentation comments

Documentation comments should be added for automatic documentation generation.  
We suggest adding documentation comments for:

- Describing the crate in general.
- Describing each modules, these doc comments goes at top level of each modules.
- Describing the `Keystore` struct, each fields and code snippets for usage.
- Describing each functions/methods and their parameters, and add code snippets for usage.

With these doc comments added you generate code documentation at compilation or directly with:

```zsh
cargo doc
```

Use `--open` option to directly open the new generated documentation in your browser.

## Unit testing

There is a not single unit test.
Add unit tests for the `keystore_create` function, edge cases should be covered.

At the end of `main.rs` add:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_keystore() {
       /* test code goes here */
    }
}
```

By following our recommendations in the [Add modularity by using the library crate for `Keystore` type and its associated methods and functions](#add-modularity-by-using-the-library-crate-for-keystore-type-and-its-associated-methods-and-functions) section, unit tests can be written per module which permit a better granularity for testing software components.

# Security

**Security audit summary**:

- [report overview](#report-overview)
- [MEDIUM: fastrand crate is a weak random number generator](#medium-fastrand-crate-is-a-weak-random-number-generator)
- [MEDIUM: A bigger key size than 128 bits can be used for the encryption](#medium-a-bigger-key-size-than-128-bits-can-be-used-for-the-encryption)
- [CRITICAL: MD5 is an insecure cryptographic hash function](#critical-md5-is-an-insecure-cryptographic-hash-function)
- [CRITICAL: Constant Initialization vector: iv = 000000000000000000000000 permit replay attack and forbidden attacks](#critical-constant-initialization-vector-iv--000000000000000000000000-permit-replay-attack-and-forbidden-attacks)
- [HIGH: deprecated and unaudited crate rust-crypto](#high-deprecated-and-unaudited-crate-rust-crypto)
- [HIGH: usage of unaudited K256 crate for secp256k1](#high-usage-of-unaudited-k256-crate-for-secp256k1)
- [CRITICAL: Reproducible secret key with a seeded random number generator](#critical-reproducible-secret-key-with-a-seeded-random-number-generator)
- [CRITICAL: HOME environnement variable can permit to find the password](#critical-home-environnement-variable-can-permit-to-find-the-password)
- [CRITICAL: The secret key and the encryption key are stored in Keystore data structure](#critical-the-secret-key-and-the-encryption-key-are-stored-in-keystore-data-structure)
- [CRITICAL: Loss of the user's keystore because it can't be deciphered anymore](#critical-loss-of-the-users-keystore-because-it-cant-be-deciphered-anymore)
- [CRITICAL: usage of secret key as additional authenticated data](#critical-usage-of-secret-key-as-additional-authenticated-data)
- [CRITICAL: encryption key collision](#critical-encryption-key-collision)
- [CRITICAL: secret key collision](#critical-secret-key-collision)

## Report overview

**CRITICAL: 9 vulnerabilities**
**HIGH: 2 vulnerabilities**
**MEDIUM: 2 vulnerabilities**
**LOW: 0 vulnerabilities**

## MEDIUM: fastrand crate is a weak random number generator

[fastrand](https://crates.io/crates/fastrand) is a simple random number generator, but not cryptographically secure.  
Use a more robust crate like [rand](https://crates.io/crates/rand).

```rust
use rand::{self, RngCore};

let mut rnd = vec![0u8; 32];
rand::thread_rng().fill_bytes(&mut rnd);
```

## MEDIUM: A bigger key size than 128 bits can be used for the encryption

line 34:

```rust
let key_size = crypto::aes::KeySize::KeySize128;
```

`aes-gcm` supports 256 bits key encryption.  
Use 256 bits key which is the size of a `SHA256` output (md5 is 128) as recommended on [MD5 is an insecure cryptographic hash function](#critical-md5-is-an-insecure-cryptographic-hash-function).  
See [deprecated and unaudited crate rust-crypto](#high-deprecated-and-unaudited-crate-rust-crypto) for a complete fix of this issue.

## CRITICAL: MD5 is an insecure cryptographic hash function

`md5` is a broken cryptographic hash function.  
Use `SHA256` instead from reputable crate like [sha2](https://crates.io/crates/sha2).

```rust
use sha2::{Digest as Dig, Sha256};

let mut hasheur = Sha256::new();
hasheur.update(password);
let digest = hasheur.finalize();
```

## CRITICAL: Constant Initialization vector: iv = 000000000000000000000000 permit replay attack and forbidden attacks

The iv is a nonce that must be unique and used 1 time only.
It permits to avoid replay attack and [forbidden attacks](https://csrc.nist.gov/csrc/media/projects/block-cipher-techniques/documents/bcm/joux_comments.pdf).  
The best solution is to upgrade the encryption algorithm to [AES-GCM-SIV](https://en.wikipedia.org/wiki/AES-GCM-SIV) to avoid nonce missuses.  
A temporarily remediation would be to use random iv or even better with a secure storage mechanism just increment the nonce after each keystore generation.
See [deprecated and unaudited crate rust-crypto](#high-deprecated-and-unaudited-crate-rust-crypto) for a workaround with random nonce generation.

## HIGH: deprecated and unaudited crate rust-crypto

[rust-crypto](https://crates.io/crates/rust-crypto) is no more maintained since 6 years.  
The usage of `aes-gcm` encryption from `rust-crypto` crate is deprecated in favor of [aes-gcm](https://crates.io/crates/aes-gcm) crate which is actively maintained by the same authors.

```rust
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, Payload},
    AeadCore, Aes256Gcm,
};

let cipher = Aes256Gcm::new_from_slice(&digest).unwrap();

let data_add = Utc::now().to_string();

let payload = Payload {
    msg: &rnd,
    aad: data_add.as_bytes(),
};

let iv = Aes256Gcm::generate_nonce(&mut OsRng); // random nonce generation

let ciphertext = cipher.encrypt(&iv, payload).unwrap();
```

## HIGH: usage of unaudited K256 crate for secp256k1

The secp256k1 elliptic curve arithmetic contained in this crate has never been independently audited!  
Use [rust-secp256k1](https://crates.io/crates/secp256k1) crate which is a rust wrapper on bitcoin's libsecp256k1.

## CRITICAL: Reproducible secret key with a seeded random number generator

The random number generator is seeded which permit a reproducible generation of the secret key.
the `rnd` buffer at line 23 will always contains the same bytes across different runs even on different environnement so the same secret key:

```zsh
$ cargo run -p  app2
Keystore { digest: "0xe9d55766648b4bb3b801186f68900f9d",
sk: "0x8e6da460de1f29a2570a7e8e3ac51a99681a5c26082f35e10dac7aac12d6d343",
pk: "00cd0993fd4a10e9826169a1ae890d0727cb27bfa5e048b6fef468fd9d4c21c3" }

# on another computer
$ cargo run -p  app2
Keystore { digest: "0xe9d55766648b4bb3b801186f68900f9d",
sk: "0x8e6da460de1f29a2570a7e8e3ac51a99681a5c26082f35e10dac7aac12d6d343", # same sk on 2nd run
pk: "00cd0993fd4a10e9826169a1ae890d0727cb27bfa5e048b6fef468fd9d4c21c3" }
```

Remove the seeding of the random number generation at line 49 or just set a seed for debug build:

```rust
fn main() {
    #[cfg(debug_assertions)]
    fastrand::seed(0);
    /* .... */
}
```

## CRITICAL: HOME environnement variable can permit to find the password

Using the `HOME` environnement variable for the encryption password is a dangerous password generation strategy.  
As `HOME` path pattern is well known for all operating system, finding the username permit to find the encryption password.  
Often inside an information system, usernames are publicly knowns for all local users.  
For remote attackers, the username can be found also by social engineering, brute force attack or by open source intelligence researches on the target's usernames chosen strategy: social media, github, post.

We recommend to let the user enter its password at keystore creation with a check on a strong password pattern (special characters, numbers, mix of upper and lower case) or use a passphrase/seed phrase for seeding a prng.

## CRITICAL: The secret key and the encryption key are stored in Keystore data structure

The secret key and encryption key must never be stored in memory or in a storage device.
the secret key has to be recovered only from a decryption key only known by the user.
Remove `sk` and `digest` field from `Keystore` data structure.

## CRITICAL: Loss of the user's keystore because it can't be deciphered anymore

`Keystore` data structure is missing critical information for the decryption of the secret key.  
The Nonce/IV, additional authenticated data, and the tag has be stored in the `Keystore` for the recovery of the secret.  
These data don't need to be kept secret and are needed for the decryption algorithm.

```rust
#[derive(Debug)]
struct Keystore {
    pk: Key,
    iv: String,
    data_add: String,
}
```

Following [The secret key is stored in Keystore data structure](#critical-the-secret-key-is-stored-in-keystore-data-structure) `sk` field has to be removed.

## CRITICAL: usage of secret key as additional authenticated data

The additional authenticated data is publicly available information that can be shared.  
Don't use the secret key as aad, instead take aad from the environnement like program version and name, timestamp, etc..
With the [chrono](https://crates.io/crates/chrono) crate we can use the current time as string for as aad:

```rust
use chrono::prelude::*;
let aad = Utc::now().to_string();
```

## CRITICAL: encryption key collision

As raw bytes of the password's digest are converted to lower case before being used as encryption key, 2 digests that differ only by their letter case will be converted to the same encryption key.

```rust
// digest1 is an hypothetical digest of a password
let digest1 = hex::decode("DEADBEEF1337").unwrap();

// digest1 is an hypothetical digest of another password
let digest2 = hex::decode("deadbeef1337").unwrap();

// to lower case conversion
let key1 = digest1.to_ascii_lowercase();
let key2 = digest2.to_ascii_lowercase();

assert_eq!(key1, key2); // true
```

remove line 29:

```rust
let key = digest.to_ascii_lowercase();
```

## CRITICAL: secret key collision

Same as [encryption key collision](#critical-encryption-key-collision).  
Don't convert to lower case the data to encrypt.
