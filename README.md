<div align="center">

# rsmanuf

[![Docs.rs Badge](https://img.shields.io/badge/docs.rs-rsmanuf-61c192.svg)](https://docs.rs/rsmanuf)
[![Crates.io Badge](https://img.shields.io/crates/v/rsmanuf.svg?color=fe7d37)](https://crates.io/crates/rsmanuf)
[![CI Badge](https://github.com/kkrypt0nn/rsmanuf/actions/workflows/ci.yml/badge.svg)](https://github.com/kkrypt0nn/rsmanuf/actions)
[![Dependency Status Badge](https://deps.rs/repo/github/kkrypt0nn/rsmanuf/status.svg)](https://deps.rs/repo/github/kkrypt0nn/rsmanuf)

[![Discord Server Badge](https://img.shields.io/discord/739934735387721768?logo=discord)](https://discord.gg/mTBrXyWxAF)
[![Last Commit Badge](https://img.shields.io/github/last-commit/kkrypt0nn/rsmanuf)](https://github.com/kkrypt0nn/rsmanuf/commits/main)
[![Conventional Commits Badge](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org/en/v1.0.0/)

</div>

---

A very simple Rust library to get the manufacturer of a specific MAC address

## Getting Started

### Installation

If you want to use this library for one of your projects, you can install it like any other Rust crate

```bash
cargo add rsmanuf
```

If you want to have the online indexing method to always use the most up to date version of the [`manuf.txt`](./src/manuf.txt) file, you need to install it with the `online` feature:

```bash
cargo add rsmanuf --features online
```

### Example Usage

To get a manufacturer, you simply need to do the following

```rust
fn main() {
    let index = rsmanuf::Index::new();
    match index.search("C4:A8:1D:73:D7:8C") {
        Ok(manuf) => {
            println!("Manufacturer: {}", manuf)
        }
        Err(error) => {
            println!("Error: {}", error)
        }
    }
}
```

To get a manufacturer by using the online index (the **`online`** feature needs to be enabled), do the following

```rust
fn main() {
    let mut index = rsmanuf::online::Index::new();
    match index.search("C4:A8:1D:73:D7:8C") {
        Ok(manuf) => {
            println!("Manufacturer: {}", manuf)
        }
        Err(error) => {
            println!("Error: {}", error)
        }
    }
}
```

## Troubleshooting

If you have problems using the crate, you can open up an [issue](https://github.com/kkrypt0nn/rsmanuf/issues) or join my [Discord server](https://discord.gg/mTBrXyWxAF).

## Contributing

People may contribute by following the [Contributing Guidelines](./CONTRIBUTING.md) and
the [Code of Conduct](./CODE_OF_CONDUCT.md)

## License

This library was made with 💜 by Krypton and is under the [MIT License](./LICENSE.md).
