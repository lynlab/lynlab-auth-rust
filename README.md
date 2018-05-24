# LYnLab Auth

[![Build Status](https://travis-ci.org/lynlab/lynlab-auth.svg?branch=master)](https://travis-ci.org/lynlab/lynlab-auth)

## Prerequisites

  - Rust 1.24+
  - [Diesel CLI](http://diesel.rs/)

## Development

```
# Set environment variables
cp .envrc.example .envrc

# Set database
diesel migration run

# Build & Run
cargo run
```
