# TIKI CLI

A quick POC of a Rust-based CLI to interact with TIKI's APIs.

## Services

These are the high-level commands:

- `account`: Interacts with the [account service](tiki/core-account-service)
- 

## Example Commands

account:
  - login
  - get-profile
  - update-profile
  - delete-account

cleanroom:
  - create
  - list
  - get

subscription:
  - list
  - get
  - create-estimate
  - purchase

## API Endpoints

Our public API is defined here: https://github.com/tiki/core-account-service/blob/main/openapi.yaml

## Notes

- "estimates" should not be subscriptions, as they are related, but independent
- Need to rename to something else like queries, models, etc
- User should be able utilize pre-built, curated models
- User should also be able to save their own collections of models
- Models are the basis of a subscription
- Purchasing a subscription gets you access to new downstream models
