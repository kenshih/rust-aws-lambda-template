# rust-aws-sqs-lambda-template

Status: _In Development_

A [cargo-generate](https://github.com/cargo-generate/cargo-generate) Template.

"hello world" which contains deps to pull off sqs message with a lambda.

no optimizations of sqs and lambda are introduced here, only the most basic connection

# table of contents

- [rust-aws-sqs-lambda-template](#rust-aws-sqs-lambda-template)
- [table of contents](#table-of-contents)
- [Getting started](#getting-started)
  - [Prerequites](#prerequites)
  - [Create your project](#create-your-project)
  - [Deploy your "hello world"](#deploy-your-hello-world)
- [notes](#notes)

# Getting started
## Prerequites
* rust
* cargo

## Create your project
1. install [cargo-generate](https://github.com/cargo-generate/cargo-generate):
```
cargo install cargo-generate
```
2. create your new project!
```
cargo generate --git https://github.com/kenshih/rust-aws-sqs-lambda-template --name <my-project-name>
```

## Deploy your "hello world"
```
# file: .local_env
export AWS_ACCESS_KEY_ID=...
export AWS_SECRET_ACCESS_KEY=...
export AWS_DEFAULT_REGION=us-east-1
```

run hello world
```
source .local_env
cargo run
```

Follow directions for setup from [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime), including MacOS setup if it applies to you.

```
## build binary executable
cargo build --release --target x86_64-unknown-linux-musl

## pack binary executable up into deployment artifact
cp ./target/x86_64-unknown-linux-musl/release/rust-aws-sqs-lambda-template ./bootstrap && zip lambda.zip bootstrap && rm bootstrap && mv lambda.zip infra/

## with default AWS profile set/matching `source .local_env` (see .local_env above)
cd infra; sam deploy; cd ..
```

# notes

- this is built using [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime)
- trying [aws-sdk-rust](https://github.com/awslabs/aws-sdk-rust) in favor of [rusoto](https://github.com/rusoto/rusoto)