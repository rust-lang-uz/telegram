use crate::{
    hooks,
    utils::{keyboard::Keyboard, message::Rustina},
};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = r#"
<b>Hello fellow (rustacean|rustina|evangelist)!</b>

This is a telegram bot created by a rustacean to help people interact with various Rust APIs way more conveniently.
"#;

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    if !hooks::is_private(bot, msg).await.unwrap() {
        return Ok(());
    }

    bot.send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard.url("Source Code", "https://github.com/rust-lang-uz/telegram");
    keyboard.url("Author", "https://github.com/orzklv")
}
