use crates_io_api::AsyncClient;
use rustina::{
    handler,
    utils::{cargo_like_log, github::GitHub, groups::Groups, resources::Resources},
};
use std::error::Error;
use teloxide::{prelude::*, update_listeners::webhooks};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    log::info!("Starting Rustina Assistant...");

    let bot = Bot::from_env();

    let groups = Groups::new();
    let github = GitHub::new();
    let crates_client = AsyncClient::new(
        "Rustina Assistant (rust@maid.uz)",
        std::time::Duration::from_millis(100),
    )
    .unwrap();
    let resources = Resources::new();

    // Dispatcher flow control
    let mut dispatcher = Dispatcher::builder(bot.clone(), handler())
        .dependencies(dptree::deps![crates_client, github, groups, resources])
        // If no handler succeeded to handle an update, this closure will be called
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        // If the dispatcher fails for some reason, execute this handler
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build();

    match std::env::var("WEBHOOK_URL") {
        Ok(v) => {
            cargo_like_log("Mode", &format!("starting webhook on {}", v));
            let addr = ([0, 0, 0, 0], 8443).into(); // port 8443
            let listener = webhooks::axum(bot, webhooks::Options::new(addr, v.parse().unwrap()))
                .await
                .expect("Couldn't setup webhook");

            dispatcher
                .dispatch_with_listener(
                    listener,
                    LoggingErrorHandler::with_custom_text(
                        "An error has occurred in the dispatcher",
                    ),
                )
                .await;
        }
        Err(_) => {
            cargo_like_log("Mode", "starting polling on localhost");
            dispatcher.dispatch().await;
        }
    }

    Ok(())
}
