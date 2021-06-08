# rust-aws-lambda-template

Use this template like this, to create your project:
![create-project-example](./create-project-example.svg)

Then, in your new project, you can deploy your AWS Lambda like this:
![sample-deploy](./sample-deploy.svg)

# table of contents

- [rust-aws-lambda-template](#rust-aws-lambda-template)
- [table of contents](#table-of-contents)
- [Overview](#overview)
- [How to use this template](#how-to-use-this-template)
  - [Prerequisites](#prerequisites)
  - [Create your project with this template!](#create-your-project-with-this-template)
- [Status of rust-aws-lambda-template](#status-of-rust-aws-lambda-template)
- [notes](#notes)

# Overview
[rust-aws-lambda-template](https://github.com/kenshih/rust-aws-lambda-template) is a [**cargo-generate**](https://github.com/cargo-generate/cargo-generate) template, which creates a rust project that can deploy a "hello world" [AWS Lambda](https://aws.amazon.com/lambda/) using [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime). Deployment is done with [aws-sam-cli](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-command-reference.html)

If you are reading this from a generated project simply overwrite this README.md with [generated.README.md](./generated.README.md) as a starting point.
# How to use this template
## Prerequisites

Prerequistes to create a project using this template:
* rust (perhaps via [rustup](https://rustup.rs/))
* [cargo](https://github.com/rust-lang/cargo/)
* [cargo-generate](https://github.com/cargo-generate/cargo-generate):
```
cargo install cargo-generate
```

## Create your project with this template!

* Run...
```
cargo generate --git https://github.com/kenshih/rust-aws-lambda-template --name <my-project-name>

# you will be prompted for:
# AWS_REGION - that your lambda will deploy in
# AWS_ACCOUNT_ID - a numeric id of our AWS account
# aws_tags - which are name/value strings like: name1=\"value1\" name2=\"value2\"
```
* This will create `<my-project-name>` similar to `cargo new`, but with a deployable lambda
* Setup and usage is covered in [generated.README.md](./generated.README.md)

# Status of rust-aws-lambda-template

- Supports lambda generation
- For requests to support additional features consider opening an [issue](https://github.com/kenshih/rust-aws-lambda-template/issues)
- TODO: Local development not supported

# notes

- this is built using [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime)