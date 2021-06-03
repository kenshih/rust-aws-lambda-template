# {{project-name}}

This project was generated from [rust-aws-lambda-template](https://github.com/kenshih/rust-aws-lambda-template), a [cargo-generate](https://github.com/cargo-generate/cargo-generate) template.

This is a deployable "hello world" [AWS Lambda](https://aws.amazon.com/lambda/).

No optimizations of lambda are introduced here, only the most basic connection to get you started.

# table of contents

- [{{project-name}}](#project-name)
- [table of contents](#table-of-contents)
- [What's in this project?](#whats-in-this-project)
- [Deploy your "hello world"](#deploy-your-hello-world)
  - [`cat deploy.sh`](#cat-deploysh)
  - [clean up](#clean-up)
- [Develop](#develop)

# What's in this project?

file/dir | description
--- | ---
generated.README.md | starter readme (TODO rename to README.md)
Cargo.toml | your rust project package management file used by [`cargo`](https://doc.rust-lang.org/cargo/)
deploy.sh | a simple script to build lambda executable and deploy using [aws-sam-cli](https://github.com/aws/aws-sam-cli) based on sample from [rust-aws-lambda-template](https://github.com/kenshih/rust-aws-lambda-template)
infra/template.yml | used by [aws-sam-cli](https://github.com/aws/aws-sam-cli) to define what is being deployed to AWS
infra/samconfig.toml | used by [aws-sam-cli](https://github.com/aws/aws-sam-cli) to configure your enviroment, profile, in order to deploy
# Deploy your "hello world"

1. Follow directions for setup from [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime), including MacOS setup if it applies to you, copied here for your convenience:
```
# macOs setup
brew install filosottile/musl-cross/musl-cross
mkdir .cargo
echo $'[target.x86_64-unknown-linux-musl]\nlinker = "x86_64-linux-musl-gcc"' > .cargo/config
```
2. run `deploy.sh` (should work on Mac/Linux, not tested on Windows)

## `cat deploy.sh`
This was added as a convenience to get you up-and-running. Consider adding this to your Makefile, preferred scripting language, etc.

```
## build binary executable
cargo build --release --target x86_64-unknown-linux-musl

## pack binary executable up into deployment artifact
cp ./target/x86_64-unknown-linux-musl/release/{{project-name}} ./bootstrap && zip lambda.zip bootstrap && rm bootstrap && mv lambda.zip infra/

## with default AWS profile set/matching `source .local_env` (see .local_env above)
cd infra; sam deploy; cd ..
```

## clean up

At the time of this writing [aws-sam-cli](https://github.com/aws/aws-sam-cli) does not have [an "undeploy/destroy" option](https://github.com/aws/aws-sam-cli/issues/789). You can use AWS's standard [aws-cli](https://github.com/aws/aws-cli) instead like this:

```
aws cloudformation delete-stack --stack-name {{project-name}}
```

# Develop

**TODO: this currently doesn't work**
```
# file: .local_env
export AWS_ACCESS_KEY_ID=...
export AWS_SECRET_ACCESS_KEY=...
export AWS_DEFAULT_REGION=us-east-1
```

run locally:
```
source .local_env
cargo run
```
