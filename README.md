# Code security and quality audit

## keystore quality and security audit

- target: [keystore](./keystore/)  
  the vulnerable program can be executed with:

```zsh
cargo run -p app2
```

- report: [keystore full audit](./keystore_full_audit.md)

- correction: [keystore-update](./keystore-update/)  
  the corrected version can be executed with:

```zsh
cargo run -p keystore-update
```

## Bitcoin Script quality audit

- target: [Script module](https://github.com/bitcoin/bitcoin/tree/master/src/script)
- report: [bitcoin code quality report](./bitcoin_script_quality_audit.md)
