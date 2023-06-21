import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { LambdaRestApi, RestApi } from 'aws-cdk-lib/aws-apigateway';
import { DockerImageCode, DockerImageFunction } from 'aws-cdk-lib/aws-lambda';


export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const lambda = new DockerImageFunction(this, "UCBotSo-Lambda", {
      code: DockerImageCode.fromImageAsset("../lambda/zapier_lambda", {}),
    });
    const lambda_api = new LambdaRestApi(this, "ucbso-lambda-api", {
      proxy: true,
      handler: lambda,
    });
  }
}
