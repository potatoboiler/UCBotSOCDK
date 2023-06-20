use std::env;

use aws_lambda_events::{event::sns::SnsEvent, ses::SimpleEmailDisposition};
use aws_sdk_lambda::Error;
use lambda_runtime::{service_fn, LambdaEvent};
use serde::{Deserialize, Serialize};
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
        .with_max_level(tracing::Level::INFO)
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
    let framework = StandardFramework::new();
    let token = env::var("UCBSO_DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_MESSAGES;
    let mut client = serenity::Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

    let (request, context) = event.into_parts();
    let announcements_channel = ChannelId(1100661212246724618 as u64);

    for record in request.records {
        let email_msg_json: serde_json::Map<String, Value> =
            serde_json::from_str(record.sns.message.as_str()).unwrap();
        let email_msg_body = email_msg_json["content"].as_str().unwrap().to_string();

        announcements_channel
            .say(&client.cache_and_http.http, email_msg_body)
            .await?;

        println!(
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
