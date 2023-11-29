use crate::utils::message::Rustina;
use teloxide::{payloads::SendMessageSetters, prelude::*, types::ParseMode};

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    bot.send_message_tf(msg.chat.id, view(msg), msg)
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}

pub fn view(msg: &Message) -> String {
    let mut message: String = String::new();

    message.push_str(&format!("<b>Chat ID:</b> {}", msg.chat.id));

    if msg.thread_id.is_some() {
        message.push_str(&format!("\n<b>Thread ID:</b> {}", msg.thread_id.unwrap()))
    }

    println!("Message: {:?}", message);

    message
}
