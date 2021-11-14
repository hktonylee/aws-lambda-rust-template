#!/usr/bin/env node
import * as cdk from '@aws-cdk/core';
import * as lambda from '@aws-cdk/aws-lambda';
import { RetentionDays } from '@aws-cdk/aws-logs';

export class RustLambdaStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // Function that calls Rust
    new lambda.Function(this, '{{project-name}}', {
      description: 'Deploying a Rust function on Lambda using the custom runtime',
      code: lambda.Code.fromAsset(
        '../target/aws-lambda-artifact.zip'
      ),
      runtime: lambda.Runtime.PROVIDED_AL2,
      architectures: [lambda.Architecture.ARM_64],
      handler: 'not.required',
      environment: {
        RUST_BACKTRACE: '1',
      },
      logRetention: RetentionDays.ONE_WEEK,
    });
  }
}

const app = new cdk.App();
new RustLambdaStack(app, '{{project-name}}', {
  env: { region: app.node.tryGetContext('region') || 'us-east-2' },
});
