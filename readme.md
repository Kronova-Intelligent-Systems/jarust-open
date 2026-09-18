# jarust-open

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![Crates.io](https://img.shields.io/crates/v/jarust-cli.svg)](https://crates.io/crates/jarust-cli)

**jarust-open** is the open-source diagnostic wedge and AST-parsing engine for the jarust ecosystem. Our main objective is to create a protocol or SDK, agentic AI service, or the optimal determined approach that efficiently rewrites memory-intensive Java services into high-performance Rust applications, eliminating JVM overhead and improving efficiency.

## Features

* **Zero-Dependency AST Parser (`jarust-ast`):** Extracts Java class hierarchies, method scopes, and object maps using a pure-Rust recursive descent lexer.
* **Diagnostic Analyzer (`jarust-analyzer`):** Flags JVM anti-patterns like heap allocation churn, lock contention, and blocking thread models.
* **Migration Urgency CLI (`jarust-cli`):** Scans any local Java repository and generates a migration score to quantify the need for an async Rust modernization.

## Installation

You can install the standalone diagnostic CLI directly from crates.io:

```bash
cargo install jarust-cli