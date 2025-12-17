#!/bin/bash

cargo sqlx prepare --workspace -- --all-targets && cargo clippy -- -D warnings
