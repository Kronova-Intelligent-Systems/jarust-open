# jarust-open

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![Crates.io](https://img.shields.io/crates/v/jarust-cli.svg)](https://crates.io/crates/jarust-cli)

**jarust-open** is the open-source diagnostic wedge and AST-parsing engine developed by **Kronova Intelligent Systems**. Our objective is to deliver an agentic AI service and SDK that efficiently rewrites memory-intensive Java services into high-performance Rust applications, eliminating JVM overhead and improving efficiency[cite: 1].

## Overview

As a standalone initiative within the Kronova Intelligent Systems portfolio, **jarust** provides the foundational tooling necessary to evaluate and execute large-scale backend modernizations. By replacing garbage-collected, JIT-compiled legacy systems with strict-ownership Rust architectures, engineering teams can drastically reduce cloud compute costs and eliminate tail-latency spikes.

## Features

* **Zero-Dependency AST Parser (`jarust-ast`):** Extracts Java class hierarchies, method scopes, and object maps using a pure-Rust recursive descent lexer.
* **Diagnostic Analyzer (`jarust-analyzer`):** Flags JVM anti-patterns like heap allocation churn, lock contention, and blocking thread models.
* **Migration Urgency CLI (`jarust-cli`):** Scans any local Java repository and generates a migration score to quantify the need for an async Rust modernization.

## Installation

You can install the standalone diagnostic CLI directly from crates.io:

```bash
cargo install jarust-cli