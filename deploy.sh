## build binary executable
cargo build --release --target x86_64-unknown-linux-musl

## pack binary executable up into deployment artifact
cp ./target/x86_64-unknown-linux-musl/release/rust-aws-sqs-lambda-template ./bootstrap && zip lambda.zip bootstrap && rm bootstrap && mv lambda.zip infra/

## with default AWS profile set/matching `source .local_env` (see .local_env above)
cd infra; sam deploy; cd ..