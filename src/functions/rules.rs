use crate::hooks;
use orzklv::telegram::keyboard::Keyboard;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = r#"
<b>Hurmatli guruh a'zosi...</b>

Iltimos qoidalarga oz bo'lsada vaqt ajratishni unutmang, bu muhim! Ushbu guruhda quyidagi harakatlar taqiqlanadi:

<code>* Besabab bir-birini kamsitish yoki so'kinish</code>
<code>* Sababsiz guruhga spam yozaverish yoki tashash</code>
<code>* So'ralgan narsani yana qayta ezmalash</code>
<code>* Administratorlarga nisbatan juddayam qattiq kritika</code>
<code>* Faoliyat ustidan shikoyat qilaverish yoki nolish</code>

<i>Hamda, bizning hamjamiyat Floss O'zbekiston jamiyati a'zosi ekan, uning qoida va standardlariga bo'ysunamiz, rad etilgan qoida va standardlar ro'yxatiga pastdagi tugmalar orqali o'tishingiz mumkin.</i>

<b>Ushbu qoidalarni doimiy tarzda buzish, bir necha ogohlantirishlirga olib keladi yoki butunlay ban!</b>
"#;

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    if !hooks::is_private(bot, msg).await.unwrap() {
        return Ok(());
    }

    bot.send_message(msg.chat.id, TEXT)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard.url("Guruhga qaytish", "https://t.me/rustlanguz").unwrap();
    keyboard.row();
    keyboard.url("Rad Etilgan Qoidalar", "https://github.com/rust-lang-uz/.github/blob/main/RULES.md").unwrap()
}
