import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as ses from 'aws-cdk-lib/aws-ses';
import * as sns from 'aws-cdk-lib/aws-sns';
import * as ses_actions from 'aws-cdk-lib/aws-ses-actions';
import * as sns_subscriptions from 'aws-cdk-lib/aws-sns-subscriptions';
import { DockerImageCode, DockerImageFunction } from 'aws-cdk-lib/aws-lambda';

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const ruleSet = new ses.ReceiptRuleSet(this, 'ForwardDiscordSES');
    const topic = new sns.Topic(this, 'ForwardDiscordSNS');
    const lambda = new DockerImageFunction(this, "UCBotSo-Lambda", {
      code: DockerImageCode.fromImageAsset("../lambda/email_lambda", {}),
    });

    const awsRule = ruleSet.addRule('Aws', {
      recipients: ['ucbso.discord@gmail.com'],
      enabled: true,
      actions: [new ses_actions.Sns({ topic })],
    });
    topic.addSubscription(new sns_subscriptions.LambdaSubscription(lambda,
      { filterPolicy: { /* TODO: only allow emails from approved members. */ } }));
  }
}
