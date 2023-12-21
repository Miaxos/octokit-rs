# octokit-rs

> machine-readable, always up-to-date

[![release](https://github.com/Miaxos/octokit-rs/actions/workflows/release.yml/badge.svg)](https://github.com/Miaxos/octokit-rs/actions/workflows/release.yml)
[![Crates.io version](https://img.shields.io/crates/v/octokit-rs.svg)](https://crates.io/crates/octokit-rs)
[![dependency status](https://deps.rs/repo/github/miaxos/octokit-rs/status.svg)](https://deps.rs/repo/github/miaxos/octokit-rs)
[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/octokit-rs)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/miaxos/octokit-rs/compare)

Automatic Octokit Types for Rust build by using [typify](https://github.com/oxidecomputer/typify).

Every hour, we fetch the JSON Schema and build the lib, if the API changed we
release a new version.

## Webhooks

We use [octokit/webhooks](https://github.com/octokit/webhooks/) to generate this
library, feel free to go check them out.
