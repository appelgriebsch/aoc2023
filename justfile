#!/usr/bin/env just --justfile

gnu_sed := if env_var("HOME") =~ "/Users.*" { "gsed" } else { "sed" }
log := "warn"

export JUST_LOG := log

set dotenv-load := true

default:
  @just --list

clean:
  @cargo clean

check:
  @cargo check
  @cargo clippy
  @cargo fmt

coverage: check
  @cargo llvm-cov nextest --lcov --output-path ./target/info.lcov

coverage-report: check
  @cargo llvm-cov nextest --html

test *ARGS: check
  @cargo nextest run --no-fail-fast {{ARGS}}

build *ARGS:
  @echo "Building on {{os()}} with {{ARGS}}".
  @cargo build {{ARGS}}

release *ARGS:
  @echo "Building on {{os()}} with {{ARGS}}".
  @cargo build --release {{ARGS}}

run target *ARGS:
  @echo "Starting {{target}} on {{os()}}".
  @cargo run --bin {{target}} {{ARGS}}

flamegraph target:
  @cargo flamegraph --profile flamegraph --root --bin {{target}} -o ./target/flamegraph/{{target}}.svg