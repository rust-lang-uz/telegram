pub mod about;
pub mod groups;
pub mod help;
pub mod inline;
pub mod latest;
pub mod start;
pub mod useful;
pub mod version;

pub use inline::inline;

use crate::utils::{github::GitHub, groups::Groups, resources::Resources};
use crate::Command;
use std::error::Error;
use teloxide::{prelude::*, types::*};

pub async fn commands(
    bot: Bot,
    _me: Me,
    msg: Message,
    cmd: Command,
    github: GitHub,
    groups: Groups,
    resources: Resources,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _ = match cmd {
        Command::Start => crate::functions::start::command(&bot, &msg).await, // done
        Command::Help => crate::functions::help::command(&bot, &msg, &cmd).await, // partially done
        Command::About => crate::functions::about::command(&bot, &msg).await, // done
        Command::Group => crate::functions::groups::command(&bot, &msg, &groups).await, // done
        Command::Latest => crate::functions::latest::command(&bot, github, &msg).await, // done
        Command::Version => crate::functions::version::command(&bot, github, &msg).await, // done
        Command::Useful => crate::functions::useful::command(&bot, &msg, &resources).await, // done
    };

    Ok(())
}

pub async fn callback(
    bot: Bot,
    q: CallbackQuery,
    github: GitHub,
    groups: Groups,
    resources: Resources,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut args: Vec<&str> = Vec::new();

    if let Some(data) = q.data.clone() {
        if data.contains('_') {
            args = data.split('_').collect();
        } else {
            args.push(&data);
        }

        let _ = match args.remove(0) {
            "group" => crate::functions::groups::callback_list(&bot, &q, &args, &groups).await,
            "detail" => crate::functions::groups::callback_detail(&bot, &q, &args).await,
            "version" => crate::functions::version::callback_list(&bot, &q, &args, github).await,
            "changelog" => {
                crate::functions::version::callback_detail(&bot, &q, &args, github).await
            }
            "useful" => crate::functions::useful::callback_list(&bot, &q, &resources).await,
            "category" => {
                crate::functions::useful::callback_category_list(&bot, &q, &args, &resources).await
            }
            "material" => {
                crate::functions::useful::callback_material_detail(&bot, &q, &args, &resources)
                    .await
            }
            _ => Ok(()),
        };
    }

    Ok(())
}
