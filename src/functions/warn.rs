// Nigga won't let me use cloned instance,
// but forcefully copy from instance that
// doesn't fucking implement Copy trait
#![allow(clippy::clone_on_copy)]

use crate::utils::topics::Topics;
use orzklv::{
    string::Transform,
    telegram::{keyboard::Keyboard, topic::Topics as TopicsTrait},
};
use std::fmt::Display;
use teloxide::{prelude::*, types::*};

static TEXT_FAIL: &str = "Ha-ha... yaxshi urinish!";
static TEXT_NON_REPLY: &str = "↪ Reply bilan ko'rsatingchi habarni!";
static NON_COMMUNITY: &str = "Ebe hay, biz O'zbek Rust hamjamiyati guruhida emasga o'xshaymiz...";

pub async fn command(bot: &Bot, msg: &Message, me: &Me, topics: &Topics) -> ResponseResult<()> {
    // if chat is not rust uzbekistan, delete
    if msg.chat.id != ChatId(-1001518595284) {
        return {
            bot.send_message_tf(msg.chat.id, NON_COMMUNITY, msg).await?;
            Ok(())
        };
    }

    // try to delete the message that tried to trigger
    let attempt = bot.delete_message(msg.chat.id, msg.id).await;
    match attempt {
        Ok(_) => {}
        Err(_) => {
            bot.send_message_tf(
                msg.chat.id,
                "Ebe hay, men habarlar o'chirish uchun yetarlicha imtiyozim yo'q!",
                msg,
            )
            .await?;
            return Ok(());
        }
    }

    let reply_to = match msg.reply_to_message() {
        Some(m) => m,
        None => {
            return {
                bot.send_message_tf(msg.chat.id, TEXT_NON_REPLY, msg)
                    .await?;

                Ok(())
            }
        }
    };

    // if there's no replied message, warn
    if msg.thread_id.is_some() && (msg.id == reply_to.id) {
        return {
            bot.send_message_tf(msg.chat.id, TEXT_NON_REPLY, msg)
                .await?;
            Ok(())
        };
    }

    // if replied person is bot itself, send a fail message
    if let Some(user) = &reply_to.from {
        if let Some(username) = &user.username {
            if username == me.username() {
                return {
                    bot.send_message_tf(msg.chat.id, TEXT_FAIL, msg).await?;
                    Ok(())
                };
            }
        }
    }

    let replied_person = match &reply_to.from {
        None => {
            bot.send_message_tf(
                msg.chat.id,
                "Hmmm, qiziq odam ekan reply qilingan odam...",
                msg,
            )
            .await?;

            return Ok(());
        }
        Some(p) => p,
    };

    let from = match &msg.from {
        None => {
            bot.send_message_tf(
                msg.chat.id,
                "Hmmm, qiziq odam ekan reply qilgan odam...",
                msg,
            )
            .await?;

            return Ok(());
        }
        Some(p) => p,
    };

    let conclusion = bot.send_message_tf(
        msg.chat.id,
        format!(
            "<b>Xo'sh, <a href=\"tg://user?id={}\">{}</a>.</b> Qaysi mavzu taraflama yozgan odam chetlashdi?",
            from.id,
            from.first_name
        ),
        msg,
    )
      .parse_mode(ParseMode::Html)
      .reply_markup(keyboard(
          topics.list(),
          from.id,
          &replied_person.id,
          &replied_person.first_name,
          &reply_to.id
      ))
      .await;

    match conclusion {
        Ok(_) => {}
        Err(_) => {
            bot.send_message(
              msg.chat.id,
              format!(
                "<b>Xo'sh, <a href=\"tg://user?id={}\">{}</a>.</b> Qaysi mavzu taraflama yozgan odam chetlashdi?",
                from.id,
                from.first_name
              )
            )
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard(
                topics.list(),
                from.id,
                &replied_person.id,
                &replied_person.first_name,
                &reply_to.id // replied message id for deleting & forwarding
            ))
            .await?;
        }
    };

    Ok(())
}

pub async fn callback(
    bot: &Bot,
    q: &CallbackQuery,
    args: &[&str],
    topics: &Topics,
) -> ResponseResult<()> {
    let message = match q.regular_message() {
        Some(m) => m,
        None => {
            return {
                bot.send_message(
                    ChatId(-1001518595284),
                    "Qaysidir thread da xabarni tushuna olmadim, akalar meni loglarim qarab ko'rasizlarmi?",
                )
                .message_thread_id(ThreadId(MessageId(255895)))
                .await?;

                Ok(())
            }
        }
    };

    let replied_person = UserId(match args[0].parse::<u64>() {
        Ok(r) => r,
        Err(_) => {
            return {
                bot.send_message_tf(
                    message.chat.id,
                    "Hmmm, qiziq odam ekan reply qilgan odam...",
                    message,
                )
                .await?;
                Ok(())
            }
        }
    });

    if q.from.id != replied_person {
        bot.answer_callback_query(q.id.clone())
            .text("Sen chaqirmadingku komandani! Nimaga o'z boshimchalik qilayabsan...")
            .show_alert(true)
            .send()
            .await?;

        return Ok(());
    }

    let title = args[1];
    let code = topics.get(title);
    let sender = (args[2], args[3]);
    let replied_message = MessageId(match args[4].parse::<i32>() {
        Ok(a) => a,
        Err(_) => {
            return {
                bot.send_message_tf(
                    message.chat.id,
                    "Reply qilingan xabarni ochib o'qiy olmadim, uzrasizlar-a?",
                    message,
                )
                .await?;

                Ok(())
            }
        }
    });

    match code {
        None => {
            bot.delete_message(message.chat.id, message.id).await?;
            bot.send_message_tf(
                message.chat.id,
                "Unaqa topic borga o'xshamaydi do'stlar...",
                message,
            )
            .await?;

            Ok(())
        }
        Some(c) => {
            bot.delete_message(message.chat.id, message.id).await?;

            let parsed_topic = match i32::try_from(c.clone()) {
                Ok(m) => m,
                Err(_) => {
                    return {
                        bot.send_message_tf(
                            message.chat.id,
                            "Xabarni o'chirishdan avval qayerga jo'natishni tushunmadim...",
                            message,
                        )
                        .await?;

                        Ok(())
                    }
                }
            };

            let forward = bot.forward_message(message.chat.id, message.chat.id, replied_message);
            match parsed_topic {
                1 => {
                    forward.await?;
                }
                _ => {
                    forward
                        .message_thread_id(ThreadId(MessageId(parsed_topic)))
                        .await?;
                }
            };

            // try to delete the replied message
            let attempt = bot.delete_message(message.chat.id, replied_message).await;
            match attempt {
                Ok(_) => {}
                Err(_) => {
                    bot.send_message_tf(
                        message.chat.id,
                        "Ebe hay, men habarlar o'chirish uchun yetarlicha imtiyozim yo'q!",
                        message,
                    )
                    .await?;
                    return Ok(());
                }
            }

            bot.send_message_tf(
                message.chat.id,
                view_detail(sender, title.to_string()),
                message,
            )
            .reply_markup(callback_keyboard(title, c))
            .parse_mode(ParseMode::Html)
            .await?;

            Ok(())
        }
    }
}

pub fn view_detail(from: (&str, &str), topic: String) -> String {
    format!(
        "<b>Hurmatli <a href=\"tg://user?id={}\">{}</a>,</b>\
        \n\n\
        Tushunishim bo'yicha siz mavzudan chetlayashayabsiz. Iltimos, \
        quyidagi tugmachani bosish orqali bizning {} guruhga o'tib oling! \
        {} guruhimizda ushbu mavzuga oid narsalar haqida suhbatlashish ruxsat etiladi. \
        Boshqalarga halaqit qilmayliga 😉\
        \n\n\
        <b>Hurmat ila, Rustacean</b>",
        from.0,
        from.1,
        topic,
        topic.capitalize()
    )
}

pub fn keyboard<T>(
    list: Vec<String>,
    owner: UserId,
    replied: &UserId,
    name: T,
    replied_message: &MessageId,
) -> InlineKeyboardMarkup
where
    T: AsRef<str> + Display,
{
    let mut keyboard = Keyboard::new();

    for (index, topic) in list.iter().enumerate() {
        keyboard.text(
            &topic.capitalize(),
            &format!(
                "warn_{}_{}_{}_{}_{}",
                owner.0, topic, replied.0, name, replied_message
            ),
        );

        if index % 2 == 1 {
            keyboard.row();
        }
    }

    keyboard.get()
}

pub fn callback_keyboard<T>(title: T, topic: &u32) -> InlineKeyboardMarkup
where
    T: AsRef<str> + Display + ToString,
{
    let mut keyboard = Keyboard::new();

    let url: String = match topic {
        0 => "https://t.me/flossuzc".to_string(),
        _ => format!("https://t.me/rustlanguz/{}", topic),
    };

    keyboard
        .url(
            &format!("{} Chat", title.to_string().capitalize()),
            &url.to_string(),
        )
        .unwrap()
}
