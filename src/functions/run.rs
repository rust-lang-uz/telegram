use crate::utils::{keyboard::Keyboard, message::Rustina};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static CONTENT: &str = r#"
<b>We tried to experiment with many types of way implementing playground and ended up with adding it as web app!</b>
"#;

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    bot.send_message_tf(msg.chat.id, CONTENT, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .reply_to_message_id(msg.id)
        .await?;

    Ok(())
}

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard.web_app("Playground", "https://play.rust-lang.org")
}
