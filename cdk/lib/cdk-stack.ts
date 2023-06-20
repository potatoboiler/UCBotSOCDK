import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as ses from 'aws-cdk-lib/aws-ses';
import * as ses_actions from 'aws-cdk-lib/aws-ses-actions';
import { DockerImageCode, DockerImageFunction } from 'aws-cdk-lib/aws-lambda';

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);
    const ruleSet = new ses.ReceiptRuleSet(this, 'ForwardDiscordSES');

    const awsRule = ruleSet.addRule('Aws', {
      recipients: ['ucbso.discord@gmail.com'],
      enabled: true,
      actions: [new ses_actions.Lambda({
        function: new DockerImageFunction(this, "UCBotSo-Lambda", {
          code: DockerImageCode.fromImageAsset("../lambda/email_lambda", {}),
        }),
      })],
    });
  }
}
