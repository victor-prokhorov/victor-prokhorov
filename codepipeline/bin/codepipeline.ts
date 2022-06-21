#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { CodepipelineStack } from '../lib/codepipeline-stack';

const app = new cdk.App();
new CodepipelineStack(app, 'CodepipelineStack', {
  env: { account: process.env.CDK_DEFAULT_ACCOUNT, region: process.env.CDK_DEFAULT_REGION },
});