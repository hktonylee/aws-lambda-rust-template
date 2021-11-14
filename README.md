Introduction
============

This template will allow you to build a CLI tool which is deployable to AWS Lambda (ARM). This is very useful when you want to develop some small tools that require scheduled run in serverless environment.

Usage
=====

In order to use this template, you need to install [cargo-generate](https://github.com/cargo-generate/cargo-generate) first. Then running the following command to use this template:

```
cargo generate --git https://github.com/hktonylee/aws-lambda-rust-template.git
```

### Run locally

To run locally as a CLI, you can use this command:

```
cargo run
```

### Deploy to AWS Lambda

To deploy to AWS Lambda, you can use this command:

```
make deploy
```

The above code deploys to AWS Lambda using CDK. You can create additional resources, say CloudWatch Events, for scheduled run.
