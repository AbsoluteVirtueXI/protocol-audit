# Code security and quality audit

## keystore

- target: [keystore](./keystore/)  
  the vulnerable program can be run with:

```zsh
cargo run -p app2
```

- report: [keystore full audit](./keystore_full_audit.md)

- correction: [keystore-update](./keystore-update/)  
  the corrected version can be run with:

```zsh
cargo run -p keystore-update
```

## bitcoin

- target: [module name](https://github.com/bitcoin/bitcoin/tree/master/src)
- report: [bitcoin code quality report](./bitcoin_quality_audit.md)
