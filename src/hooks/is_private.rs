use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

use orzklv::telegram::{keyboard::Keyboard, timer::Timer, topic::Topics};

static TEXT: &str = "⚠️ <b>Bu komanda faqat shaxsiy chat uchun!</b>";

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Keyboard = Keyboard::new();
    keyboard
        .url("Shaxsiy Chat", "https://t.me/rustaceanbot")
        .unwrap()
}

pub async fn is_private(bot: &Bot, msg: &Message) -> ResponseResult<bool> {
    if msg.chat.is_private() {
        return Ok(true);
    }

    match bot.delete_message(msg.chat.id, msg.id).await {
        Ok(_) => {}
        Err(_) => {}
    };

    let message = bot
        .send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    bot.delete_timer(message.chat.id, message.id, 10)
        .await
        .await?;

    Ok(false)
}
