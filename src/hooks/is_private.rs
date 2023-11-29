use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

use crate::utils::{
    keyboard::Keyboard,
    message::{delete_timer, Rustina},
};

static TEXT: &str = "⚠️ <b>Bu komanda faqat shaxsiy chat uchun!</b>";

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Keyboard = Keyboard::new();
    keyboard.url("Shaxsiy Chat", "https://t.me/rustinabot")
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

    delete_timer(bot, &message, 10).await?;

    Ok(false)
}
