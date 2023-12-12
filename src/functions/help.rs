use super::start::keyboard;
use crate::{utils::message::Rustina, Command};
use teloxide::{payloads::SendMessageSetters, prelude::*, types::ParseMode};

static TEXT: &[(&str, &str)] = &[
    ("help", "show this message"),
    ("about", "briefly about this bot"),
    ("latest", "get short info about latest version"),
    ("version", "get information about specific version"),
    ("group", "rust communities"),
    ("run &lt;code&gt;", "run code and show stout & sterr"),
    ("useful", "useful resources"),
];

pub async fn command(bot: &Bot, msg: &Message, _cmd: &Command) -> ResponseResult<()> {
    let mut text = String::new();

    text.push_str("<b>List of available commands:</b>\n\n");

    for cmd in TEXT {
        text.push('/');
        text.push_str(cmd.0);
        text.push_str(" - ");
        text.push_str(format!("<code>{text}</code>", text = cmd.1).as_str());
        text.push('\n');
    }

    bot.send_message_tf(msg.chat.id, text, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}
