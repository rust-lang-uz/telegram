use crate::utils::{keyboard::Keyboard, message::Rustina};
use teloxide::{prelude::*, types::*};

static TEXT_FAIL: &str = "Ha-ha... yaxshi urinish!";
static TEXT_NON_REPLY: &str = "↪ Reply bilan ko'rsatingchi habarni!";

pub async fn command(bot: &Bot, msg: &Message, me: &Me) -> ResponseResult<()> {
    if msg.reply_to_message().is_none() {
        return {
            bot.send_message_tf(msg.chat.id, TEXT_NON_REPLY, msg)
                .await?;
            Ok(())
        };
    }

    // if replied person is bot itself, send fail message
    if let Some(user) = msg.reply_to_message().as_ref().unwrap().from() {
        if user.username.is_some() && user.username.clone().unwrap() == me.username() {
            return {
                bot.send_message_tf(msg.chat.id, TEXT_FAIL, msg).await?;
                Ok(())
            };
        }
    }

    bot.delete_message(msg.chat.id, msg.id).await?;
    bot.delete_message(msg.chat.id, msg.reply_to_message().unwrap().id)
        .await?;

    bot.send_message_tf(msg.chat.id, view(msg.reply_to_message().unwrap()), msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}

pub fn view(msg: &Message) -> String {
    format!(
        "<b>Hurmatli <a href=\"tg://user?id={}\">{}</a>,</b>\
        \n\n\
        Tushunishim bo'yicha siz mavzudan chetlayashayabsiz. Iltimos, \
        quyidagi tugmachani bosish orqali bizning offtop guruhga o'tib oling! \
        Offtopic guruhimizda istalgan mavzuda suhbatlashish ruxsat etiladi. Boshqalarga halaqit qilmayliga 😉\
        \n\n\
        <b>Hurmat ila, Rustina (Rastina)</b>",
        msg.from().unwrap().id,
        msg.from().unwrap().first_name
    )
}

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard.url("Offtopic", "https://t.me/rustlanguz/9400")
}
