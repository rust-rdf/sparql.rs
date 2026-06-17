# SPARQL.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/sparql)](https://crates.io/crates/sparql)
[![Documentation](https://docs.rs/sparql/badge.svg)](https://docs.rs/sparql)

**SPARQL.rs is a dataflow implementation of the SPARQL graph query language
for RDF knowledge graphs.**

> [!TIP]
> 🚧 _We are building in public. This is presently under heavy construction._

<sub>

[[Features](#-features)] |
[[Prerequisites](#%EF%B8%8F-prerequisites)] |
[[Installation](#%EF%B8%8F-installation)] |
[[Examples](#-examples)] |
[[Reference](#-reference)] |
[[Development](#%E2%80%8D-development)]

</sub>

## ✨ Features

- 100% pure and safe Rust with minimal dependencies and no bloat.
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- Cuts red tape: 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add sparql
```

### Installation in `Cargo.toml`

Enable all default features:

```toml
[dependencies]
sparql = { version = "0.0" }
```

Enable only specific features:

```toml
[dependencies]
sparql = { version = "0.0", default-features = false, features = ["serde"] }
```

## 👉 Examples

### Importing the Library

```rust
use sparql::{algebra, engine, parser};
```

### Parsing a SPARQL Query

```rust
let query = sparql::parse("SELECT * WHERE { ?s ?p ?o }").expect("should be valid query");
```

## 📚 Reference

[docs.rs/sparql](https://docs.rs/sparql)

### Core Packages

| Package | Crate | Docs |
| :------ | :---- | :--- |
| [sparql](https://github.com/rust-rdf/sparql.rs/tree/master/lib/sparql#readme) | [![Package](https://img.shields.io/crates/v/sparql)](https://crates.io/crates/sparql) | [![Documentation](https://img.shields.io/docsrs/sparql?label=docs.rs)](https://docs.rs/sparql) |
| [sparql-algebra](https://github.com/rust-rdf/sparql.rs/tree/master/lib/sparql-algebra#readme) | [![Package](https://img.shields.io/crates/v/sparql-algebra)](https://crates.io/crates/sparql-algebra) | [![Documentation](https://img.shields.io/docsrs/sparql-algebra?label=docs.rs)](https://docs.rs/sparql-algebra) |
| [sparql-derive](https://github.com/rust-rdf/sparql.rs/tree/master/lib/sparql-derive#readme) | [![Package](https://img.shields.io/crates/v/sparql-derive)](https://crates.io/crates/sparql-derive) | [![Documentation](https://img.shields.io/docsrs/sparql-derive?label=docs.rs)](https://docs.rs/sparql-derive) |
| [sparql-engine](https://github.com/rust-rdf/sparql.rs/tree/master/lib/sparql-engine#readme) | [![Package](https://img.shields.io/crates/v/sparql-engine)](https://crates.io/crates/sparql-engine) | [![Documentation](https://img.shields.io/docsrs/sparql-engine?label=docs.rs)](https://docs.rs/sparql-engine) |
| [sparql-parser](https://github.com/rust-rdf/sparql.rs/tree/master/lib/sparql-parser#readme) | [![Package](https://img.shields.io/crates/v/sparql-parser)](https://crates.io/crates/sparql-parser) | [![Documentation](https://img.shields.io/docsrs/sparql-parser?label=docs.rs)](https://docs.rs/sparql-parser) |

### Feature Flags

#### Interoperability

| Feature      | Version | Summary |
| :----------- | :------ | :------ |
| `datafusion` | 50.0    | Planned interop with Apache DataFusion and `rdf-fusion`
| `oxrdf`      | 0.3     | Implements `From<T> for oxrdf::T`, `From<oxrdf::T> for T`
| `rudof`      | 0.1     | Planned interop with `rudof`
| `serde`      | 1.0     | Derives `serde::{Serialize, Deserialize}`
| `sophia`     | 0.10    | Implements `From<T> for sophia::T`, `From<sophia::T> for T`

## 👨‍💻 Development

```bash
git clone https://github.com/rust-rdf/sparql.rs.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/rust-rdf/sparql.rs&text=SPARQL.rs)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/rust-rdf/sparql.rs&title=SPARQL.rs)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/rust-rdf/sparql.rs&t=SPARQL.rs)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/rust-rdf/sparql.rs)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/rust-rdf/sparql.rs)

[feature flags]: https://github.com/rust-rdf/sparql.rs/blob/master/lib/sparql/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[RDF]: https://www.w3.org/TR/rdf12-concepts/
[Rust]: https://rust-lang.org
