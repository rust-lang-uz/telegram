use teloxide::{
    prelude::*,
    requests::JsonRequest,
    types::*,
    {payloads, payloads::*},
};

pub trait Rustina {
    type Err: std::error::Error + Send;
    type SendMessageTF: Request<Payload = SendMessage, Err = Self::Err>;

    /// For Telegram documentation see [`SendMessage`].
    fn send_message_tf<C, T>(&self, chat_id: C, text: T, message: &Message) -> Self::SendMessageTF
    where
        C: Into<Recipient>,
        T: Into<String>;
}

impl Rustina for Bot {
    type Err = teloxide::errors::RequestError;
    type SendMessageTF = JsonRequest<payloads::SendMessage>;

    fn send_message_tf<C, T>(&self, chat_id: C, text: T, message: &Message) -> Self::SendMessageTF
    where
        C: Into<Recipient>,
        T: Into<String>,
    {
        match message.thread_id {
            Some(thread_id) => {
                Self::SendMessageTF::new(self.clone(), payloads::SendMessage::new(chat_id, text))
                    .message_thread_id(thread_id)
            }
            None => {
                Self::SendMessageTF::new(self.clone(), payloads::SendMessage::new(chat_id, text))
            }
        }
    }
}

// Delete a message after a certain time
pub async fn delete_timer(bot: &Bot, message: &Message, timer: u64) -> ResponseResult<()> {
    let bot = bot.clone();
    let message = message.clone();

    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(timer)).await;
        match bot.delete_message(message.chat.id, message.id).await {
            Ok(_) => {}
            Err(_) => {}
        };
    });

    Ok(())
}
