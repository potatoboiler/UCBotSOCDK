use std::env;

use aws_lambda_events::event::sns::SnsEvent;
use aws_sdk_lambda::Error;
use lambda_runtime::{service_fn, LambdaEvent};
use serde::Serialize;
use serde_json::Value;
use serenity::{
    framework::{standard::macros::group, StandardFramework},
    model::prelude::ChannelId,
    prelude::{EventHandler, GatewayIntents},
};

#[group]
struct General;
struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    _ = lambda_runtime::run(service_fn(handler_fn)).await;
    Ok(())
}

#[derive(Serialize)]
struct Response {
    req_id: String,
}

async fn handler_fn(event: LambdaEvent<SnsEvent>) -> Result<Response, Box<dyn std::error::Error>> {
    // TODO: refactor to make a direct API call instead of doing this.

    let framework = StandardFramework::new();
    let token = env::var("UCBSO_DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_MESSAGES;
    let client = serenity::Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    tracing::info!("Created client successfully.");

    tracing::info!("Processing record {:#?}", event);

    let (request, context) = event.into_parts();
    let announcements_channel = ChannelId(1100661212246724618 as u64);

    for record in request.records {
        tracing::info!("Processing record: {:?}", record);
        tracing::info!(
            "Processing email with subject: {}",
            record
                .sns
                .subject
                .clone()
                .unwrap_or("Default (null) subject".to_string())
        );

        // Examples: https://docs.aws.amazon.com/ses/latest/dg/receiving-email-notifications-examples.html
        // Format: https://docs.aws.amazon.com/ses/latest/dg/receiving-email-notifications-contents.html
        let email_msg_json: Value = serde_json::from_str(record.sns.message.as_str()).unwrap();
        let email_msg_body = email_msg_json["content"].as_str().unwrap().to_string();
        // tracing::info!("{:#?}", email_msg_json);
        tracing::info!("Email body: {:#?}", email_msg_body);

        announcements_channel
            .say(&client.cache_and_http.http, email_msg_body)
            .await?;

        tracing::info!(
            "Posted email with subject: {}",
            record
                .sns
                .subject
                .unwrap_or("Default (null) subject".to_string())
        );
    }

    Ok(Response {
        req_id: context.request_id,
    })
}
