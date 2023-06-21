use std::env;

use lambda_http::{IntoResponse, Request, RequestPayloadExt};
use lambda_runtime::service_fn;
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
async fn main() -> Result<(), anyhow::Error> {
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

    _ = lambda_http::run(service_fn(handler_fn)).await;
    Ok(())
}

async fn handler_fn(event: Request) -> Result<impl IntoResponse, anyhow::Error> {
    let token = env::var("UCBSO_DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_MESSAGES;
    let client = serenity::Client::builder(token, intents)
        .event_handler(Handler)
        .framework(StandardFramework::new())
        .await
        .expect("Error creating client");
    tracing::info!("Created client successfully.");

    tracing::info!("Processing event data {:?}", event);
    let json = event.payload::<Value>().unwrap().unwrap();
    let json = json.as_object().unwrap();
    // TODO: parse json["body_html"] ?
    let email_body = snailquote::unescape(json["body_plaintext"].as_str().unwrap()).unwrap();
    let text_file = format!(
        "From: {}\nSubject: {}\n===\n{}",
        json["from"].as_str().unwrap(),
        json["subject"].as_str().unwrap(),
        email_body
    );

    let announcements_channel = ChannelId(1100661212246724618 as u64);
    announcements_channel
        .send_files(
            &client.cache_and_http.http,
            vec![(text_file.as_bytes(), "email.txt")],
            |m| m.content("Fresh email, hot off the presses!"),
        )
        .await?;

    Ok("".into_response().await)
}

#[cfg(test)]
#[test]
fn testing_json() {
    const S: &str = r##"{"from": "UCBSO -ucbso.discord@gmail.com", "subject": "Re: test", "body_plaintext": "test\r\n\r\nOn Wed, Jun 21, 2023 at 3:28\u202fAM UCBSO <ucbso.discord@gmail.com> wrote:\r\n\r\n> test\r\n>\r\n> On Tue, Jun 20, 2023 at 2:46\u202fAM UCBSO <ucbso.discord@gmail.com> wrote:\r\n>\r\n>> test\r\n>>\r\n>> On Tue, Jun 20, 2023 at 2:43\u202fAM UCBSO <ucbso.discord@gmail.com> wrote:\r\n>>\r\n>>> test\r\n>>>\r\n>>\r\n", "body_html": "<div dir=\"ltr\">test</div><br><div class=\"gmail_quote\"><div dir=\"ltr\" class=\"gmail_attr\">On Wed, Jun 21, 2023 at 3:28\u202fAM UCBSO &lt;<a href=\"mailto:ucbso.discord@gmail.com\">ucbso.discord@gmail.com</a>&gt; wrote:<br></div><blockquote class=\"gmail_quote\" style=\"margin:0px 0px 0px 0.8ex;border-left:1px solid rgb(204,204,204);padding-left:1ex\"><div dir=\"ltr\">test</div><br><div class=\"gmail_quote\"><div dir=\"ltr\" class=\"gmail_attr\">On Tue, Jun 20, 2023 at 2:46\u202fAM UCBSO &lt;<a href=\"mailto:ucbso.discord@gmail.com\" target=\"_blank\">ucbso.discord@gmail.com</a>&gt; wrote:<br></div><blockquote class=\"gmail_quote\" style=\"margin:0px 0px 0px 0.8ex;border-left:1px solid rgb(204,204,204);padding-left:1ex\"><div dir=\"ltr\">test</div><br><div class=\"gmail_quote\"><div dir=\"ltr\" class=\"gmail_attr\">On Tue, Jun 20, 2023 at 2:43\u202fAM UCBSO &lt;<a href=\"mailto:ucbso.discord@gmail.com\" target=\"_blank\">ucbso.discord@gmail.com</a>&gt; wrote:<br></div><blockquote class=\"gmail_quote\" style=\"margin:0px 0px 0px 0.8ex;border-left:1px solid rgb(204,204,204);padding-left:1ex\"><div dir=\"ltr\">test</div>\r\n</blockquote></div>\r\n</blockquote></div>\r\n</blockquote></div>\r\n"} "##;
    println!(
        "{}",
        snailquote::unescape(
            serde_json::from_str::<Value>(S).unwrap()["body_plaintext"]
                .as_str()
                .unwrap()
        )
        .unwrap()
    );
}
