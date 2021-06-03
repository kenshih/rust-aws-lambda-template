# rust-aws-lambda-template

[rust-aws-lambda-template](https://github.com/kenshih/rust-aws-lambda-template) is a [cargo-generate](https://github.com/cargo-generate/cargo-generate) template, which creates a rust project that can deploy a "hello world" [AWS Lambda](https://aws.amazon.com/lambda/) using [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime).

If you are reading a generated project simply overwrite this README.md with [generated.README.md](./generated.README.md) as a starting point.

# table of contents

- [rust-aws-lambda-template](#rust-aws-lambda-template)
- [table of contents](#table-of-contents)
- [How to use this template](#how-to-use-this-template)
  - [Prerequisites](#prerequisites)
  - [Create your project wit this template!](#create-your-project-wit-this-template)
- [Status of rust-aws-lambda-template](#status-of-rust-aws-lambda-template)
- [notes](#notes)

# How to use this template
## Prerequisites

* rust (perhaps via [rustup](https://rustup.rs/))
* [cargo](https://github.com/rust-lang/cargo/)

## Create your project wit this template!
1. install [cargo-generate](https://github.com/cargo-generate/cargo-generate):
```
cargo install cargo-generate
```
2. create your new project!
```
cargo generate --git https://github.com/kenshih/rust-aws-lambda-template --name <my-project-name>

# you will be prompted for:
# AWS_REGION - that your lambda will deploy in
# AWS_ACCOUNT_ID - a numeric id of our AWS account
# aws_tags - which are name/value strings like: name1=\"value1\" name2=\"value2\"
```

# Status of rust-aws-lambda-template

- Supports lambda generation
- For requests to support additional features consider opening an [issue](https://github.com/kenshih/rust-aws-lambda-template/issues)
- TODO: Local development not supported
- TODO: doc: gif of how to
- TODO: doc: tour of generated items

# notes

- this is built using [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime)
- using [aws-sdk-rust](https://github.com/awslabs/aws-sdk-rust) in favor of [rusoto](https://github.com/rusoto/rusoto)