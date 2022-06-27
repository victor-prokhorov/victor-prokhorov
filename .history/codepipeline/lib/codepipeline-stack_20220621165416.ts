import { CfnOutput, Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { Pipeline, Artifact } from 'aws-cdk-lib/aws-codepipeline';
import {
  CodeBuildAction,
  CodeStarConnectionsSourceAction,
  S3DeployAction,
} from 'aws-cdk-lib/aws-codepipeline-actions';
import {
  PipelineProject,
  BuildSpec,
  LinuxBuildImage,
} from 'aws-cdk-lib/aws-codebuild';
import { Bucket } from 'aws-cdk-lib/aws-s3';

const connectionArn = process.env.CONNECTION_ARN;
export class CodepipelineStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const pipeline = new Pipeline(this, 'codepipeline', {
      pipelineName: 'codepipeline',
    });


    const sourceName = 'source';
    const sourceStage = pipeline.addStage({ stageName: sourceName });
    const sourceArtifact = new Artifact();
    if (connectionArn === undefined) throw new Error();
    sourceStage.addAction(
      new CodeStarConnectionsSourceAction({
        actionName: sourceName,
        repo: 'victor-prokhorov',
        owner: 'victor-prokhorov',
        branch: 'main',
        output: sourceArtifact,
        connectionArn,
        triggerOnPush: true,
        codeBuildCloneOutput: true,
      }),
    );

    const testName = 'test';
    const test = pipeline.addStage({ stageName: testName });
    const testArtifact = new Artifact();
    test.addAction(
      new CodeBuildAction({
        actionName: testName,
        project: new PipelineProject(this, testName, {
          projectName: testName,
          environment: {
            buildImage: LinuxBuildImage.STANDARD_5_0,
          },
          buildSpec: BuildSpec.fromSourceFilename(
            './codepipeline/lib/testspec.yaml',
          ),
        }),
        input: sourceArtifact,
        outputs: [testArtifact],
      }),
    );

    const buildName = 'build';
    const build = pipeline.addStage({ stageName: buildName });
    const buildArtifact = new Artifact();
    build.addAction(
      new CodeBuildAction({
        actionName: buildName,
        project: new PipelineProject(this, buildName, {
          projectName: buildName,
          environment: {
            buildImage: LinuxBuildImage.STANDARD_5_0,
          },
          buildSpec: BuildSpec.fromSourceFilename(
            './codepipeline/lib/buildspec.yaml',
          ),
        }),
        input: testArtifact,
        outputs: [buildArtifact],
      }),
    );

    const deployName = 'deploy';
    const deploy = pipeline.addStage({ stageName: deployName });
    const bucket = new Bucket(this, 'bucket', {
      websiteIndexDocument: 'index.html',
      publicReadAccess: true,
    });
    deploy.addAction(
      new S3DeployAction({
        actionName: deployName,
        input: buildArtifact,
        bucket,
      }),
    );

    new CfnOutput(this, 'bucket-website-url', {
      value: bucket.bucketWebsiteUrl,
    });
  }
}
