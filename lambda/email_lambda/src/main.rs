use std::env;

use aws_sdk_lambda::Error;
use lambda_runtime::{service_fn, LambdaEvent};
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize)]
struct Request {
    message: String,
}
#[derive(Serialize)]
struct Response {
    req_id: String,
}

async fn handler_fn(event: LambdaEvent<Request>) -> Result<Response, Box<dyn std::error::Error>> {
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
    ChannelId(1100661212246724618 as u64)
        .say(&client.cache_and_http.http, request.message)
        .await?;

    Ok(Response {
        req_id: context.request_id,
    })
}
