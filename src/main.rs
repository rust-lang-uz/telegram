use bot::bot::dispatch;
use bot::config::{Config, Field};
use bot::utils::{clog, github::GitHub, topics::Topics};
use bot::utils::{groups::Groups, resources::Resources};
use bot::{Cli, Commands};
use clap::Parser;
use crates_io_api::AsyncClient;
use std::error::Error;
use teloxide::{prelude::*, update_listeners::webhooks};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Starter packs
    pretty_env_logger::init();
    log::info!("Starting Rustina Assistant...");

    // Global instances
    let groups = Groups::new();
    let topics = Topics::new();
    let resources = Resources::new();
    let mut config = Config::default();
    let crates_client = AsyncClient::new(
        "Rustacean Assistant (support@floss.uz)",
        std::time::Duration::from_millis(100),
    )
    .unwrap();

    // Args
    let args = Cli::parse();

    match args.command {
        Commands::Polling { token, github } => {
            match config.read(token, Field::Token) {
                Ok(_) => clog("Config", "Successfully read the token variable"),
                Err(e) => panic!("{}", e),
            };

            match config.read(github, Field::GitHub) {
                Ok(_) => clog("Config", "Successfully read the github token variable"),
                Err(e) => panic!("{}", e),
            };

            let bot = Bot::new(config.token);
            let github = GitHub::new(Some(config.github));

            // Dependencies
            let deps = dptree::deps![crates_client, github, groups, resources, topics];

            let mut dispatcher = dispatch(&bot, deps);

            clog("Mode", "starting polling on localhost");
            dispatcher.dispatch().await;

            Ok(())
        }
        Commands::Webhook {
            token,
            github,
            domain,
            port,
        } => {
            match config.read(token, Field::Token) {
                Ok(_) => clog("Config", "Successfully read the token variable"),
                Err(e) => panic!("{}", e),
            };

            match config.read(github, Field::GitHub) {
                Ok(_) => clog("Config", "Successfully read the github token variable"),
                Err(e) => panic!("{}", e),
            };

            match config.set(format!("https://{}", domain), Field::Domain) {
                Ok(_) => clog("Config", "Successfully set the domain variable"),
                Err(e) => panic!("{}", e),
            }

            let bot = Bot::new(config.token);
            let github = GitHub::new(Some(config.github));

            // Dependencies
            let deps = dptree::deps![crates_client, github, groups, resources, topics];

            let mut dispatcher = dispatch(&bot, deps);

            let addr = ([127, 0, 0, 1], port.unwrap_or(8445)).into(); // port 8445

            let listener = webhooks::axum(
                bot,
                webhooks::Options::new(addr, config.domain.parse().unwrap()),
            )
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

            Ok(())
        }
        Commands::Env => {
            let bot = Bot::from_env();
            let github = GitHub::new(std::env::var("GITHUB_TOKEN").ok());

            // Dependencies
            let deps = dptree::deps![crates_client, github, groups, resources, topics];

            let mut dispatcher = dispatch(&bot, deps);

            match std::env::var("WEBHOOK_URL") {
                Ok(v) => {
                    clog("Mode", &format!("starting webhook on {}", v));

                    let port: u16 = std::env::var("PORT")
                        .unwrap_or("8445".to_string())
                        .parse()
                        .unwrap_or(8445);

                    let addr = ([0, 0, 0, 0], port).into();

                    let listener =
                        webhooks::axum(bot, webhooks::Options::new(addr, v.parse().unwrap()))
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
                    clog("Mode", "starting polling on localhost");
                    dispatcher.dispatch().await;
                }
            }

            Ok(())
        }
    }
}
