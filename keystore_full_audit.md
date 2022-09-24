# **keystore quality and security code audit**

# Executive summary

This report is a quality and security audit of the crate [keystore](./keystore).  
We provide first a code quality analysis without consideration of security flaws, as quality applied even for an insecure code.  
We follow then with a security analysis of the code.  
A corrected version of the crate `keystore` with quality and security considerations applied can be found at [keystore-update](./keystore-update).

# Quality audit

Without an internal coding style and convention written for this project, we assume that rust official and community known conventions should be followed for rust syntax and design.  
We use an hybrid approach with a manual audit based on our Rust and software architecture expertise and an automated analysis based on [clippy](https://github.com/rust-lang/rust-clippy) linter for highlighting common mistake in Rust code.  
For more details on Rust common conventions and Rust style guide please read:  
https://rust-lang.github.io/api-guidelines  
https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md

**Code quality audit summary**:

- [Cargo.toml](#cargotoml)
  - [[package] section](#package-section)
  - [[dependencies] section](#dependencies-section)
- [Encapsulate `keystore_create` in `Keystore` struct as an associated function.](#encapsulate-keystorecreate-in-keystore-struct-as-an-associated-function)
- [Add modularity by using the library crate for `Keystore` type and its associated methods and functions.](#add-modularity-by-using-the-library-crate-for-keystore-type-and-its-associated-methods-and-functions)
- [Inconsistency in code formatting](#inconsistency-in-code-formatting)
- [Unused import `AeadDecryptor`](#unused-import-aeaddecryptor)
- [Import of `std::iter` and relative path `iter::repeat` and `iter::repeat_with`](#import-of-stditer-and-relative-path-iterrepeat-and-iterrepeatwith)
- [Unused mutable let bindings](#unused-mutable-let-bindings)
- [Restricted portability due to the usage of `HOME` environment variable](#restricted-portability-due-to-the-usage-of-home-environment-variable)
- [Unnecessary `let` binding `k`](#unnecessary-let-binding-k)
- [Redundant program termination scheme with a bad exit code](#redundant-program-termination-scheme-with-a-bad-exit-code)
- [Comments](#comments)
- [Documentation comments](#documentation-comments)
- [Unit testing](#unit-testing)

## Cargo.toml

### [package] section.

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

### [dependencies] section.

- Remove commented dependency at line 10 and the useless blank line at line 11:

```toml
#primitive-types = "0.10.0"

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

`k256` is an unaudited package. We just showed how to get the last version of the package, but for security reason, and particularly for application using cryptography you should use an alternative. See our security analysis below for more information. TODO LINK TO SEC HERE.

## Encapsulate `keystore_create` in `Keystore` struct as an associated function.

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

## Add modularity by using the library crate for `Keystore` type and its associated methods and functions.

Library code and binary code should be separated to enhance modularity, readability and maintenance.
`main.rs` should only contain minimum code to launch the program, and import modules, types and functionalities from the library crate (`lib.rs`).

Put `Keystore` struct and implementation, and all its related dependencies, in a file named `lib.rs` or in a dedicated submodule.
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

## Inconsistency in code formatting

Readability can be improved by consistency in formatting and a 4 spaces indentation.
Configure your IDE and install [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension or install and use the [rustfmt](https://github.com/rust-lang/rustfmt) command line.

```zsh
$ rustup component add rustfmt
$ cargo fmt
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

Same pattern should be applied for the `std::process` module and the function `exit`, but as we will see in [Redundant program termination scheme with a bad exit code](#redundant-program-termination-scheme-with-a-bad-exit-code), we recommend to remove the import of `std::process` and the call to the `exit` function.

## Unused mutable let bindings.

The keyword `mut` is used for let bindings which don't need to be mutable.  
Variables `key`, `data` and `iv` do not need to be mutable, remove `mut` keyword from the associated `let` bindings.  
TODO LINK SEC: more info on `iv` meaning, type (static?) and security issue;

## Restricted portability due to the usage of `HOME` environment variable.

At line 50 the usage of the `HOME` environment variable restrict the program to run correctly only on Linux/Unix/BSD based operating systems as this environment variable is only set by default on those OS.  
Use another environnement variable, and preferably an environnement variable created and set by the program itself.

**Important Security Warning**:  
The line 50 `let password = env::var("HOME").unwrap();` introduces critical security vulnerabilities.  
Please check TODO LINK SEC before working around this quality issue.

## Unnecessary `let` binding `k`.

The `let` binding `k` defined at line 40 is returned directly at line 45 within `keystore_create` function.  
It is extraneous code. Remove it to make your code more rusty and return directly the `Keystore` instantiation expression.

```rust
Keystore {
            digest: format!("0x{}", hex::encode(digest.to_vec())),
            sk: format!("0x{}", hex::encode(sk)),
            pk: hex::encode(output),
        }
```

## Redundant program termination scheme with a bad exit code.

Remove import `use std::process::exit;` at line 8 and remove the call of `exit` function at line 53.  
The usage of the `std::process::exit` function, at line 53, is not needed as it is called as the last statement of the program.  
`std::process:exit` is a very good option for terminating a program based on some conditions for an early exit with a variable exit code, which is unnecessary in the actual program.  
Moreover the exit code `1` is a common convention for a catchall for general errors, which is not the case in the current program as it exits successfully without internal errors.

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

By following our recommendations in the `Modularity` section, unit tests can be written per module which permit a better granularity for testing software components.

# Security

## report overview
