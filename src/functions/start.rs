use crate::utils::{keyboard::Keyboard, message::Rustina};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = r#"
<b>Hello fellow (rustacean|rustina|rust evangelist)!</b>

Glad that you decided to give a try this bot. This bot helps you to do some rusty things without leaving telegram social messenger and interact with various APIs way more conveniently.
"#;

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    bot.send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard.url("Rust", "https://rust-lang.org");
    keyboard.url("GitHub", "https://github.com/rust-lang");
    keyboard.row();
    keyboard.url("Our Team", "https://github.com/rust-lang-uz");
    keyboard.url("Rustacean", "https://rustacean.net")
}
