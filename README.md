# TIKI CLI

[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)

A quick POC of a Rust-based CLI to interact with TIKI's APIs.

This is in early development and is not yet stable.

# Usage

Build with `cargo build`.

Set your TIKI API Key as the `TIKI_API_KEY` env variable.

Help is available via the `help` subcommand (or the -h / --help) flags
for top-level commands, as well as all available subcommands.

```
$ tiki --help
Usage: tiki --api-key <API_KEY> <COMMAND>

Commands:
  account       
  cleanroom     
  query         
  subscription  
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Example Commands

account:
  - login
  - get-profile
  - update-profile

cleanroom:
  - create-cleanroom
  - list-cleanrooms
  - get-cleanroom
  - delete-cleanroom

query:
  - create-estimate
  - list-estimates

subscription:
  - list-subscriptions
  - get-subscription
  - purchase-subscription
  - pause-subscription
  - delete-subscription

## API Endpoints

Our public API is defined here: https://github.com/tiki/core-account-service/blob/main/openapi.yaml

## Notes

- "estimates" should not be subscriptions, as they are related, but independent
- Need to rename to something else like queries, models, etc
- User should be able utilize pre-built, curated models
- User should also be able to save their own collections of models
- Models are the basis of a subscription
- Purchasing a subscription gets you access to new downstream models
