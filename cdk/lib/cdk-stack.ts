import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as ses from 'aws-cdk-lib/aws-ses';
import * as sns from 'aws-cdk-lib/aws-sns';
import * as ses_actions from 'aws-cdk-lib/aws-ses-actions';
import * as sns_subscriptions from 'aws-cdk-lib/aws-sns-subscriptions';

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const topic = new sns.Topic(this, 'ForwardDiscordSNS');
    // UCBotSO URL endpoint shouhld go below
    topic.addSubscription(new sns_subscriptions.UrlSubscription('https://google.com/'));

    const ruleSet = new ses.ReceiptRuleSet(this, 'ForwardDiscordSES');
    const awsRule = ruleSet.addRule('Aws', {
      recipients: ['ucbso.discord@gmail.com'],
      enabled: true,
      actions: [new ses_actions.Sns({
        topic
      })],
    });
  }
}
