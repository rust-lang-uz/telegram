use crate::hooks;
use crate::utils::keyboard::Keyboard;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static ROADMAP: &str = include_str!("../../data/roadmap.md");

static LINKS: &[(&str, &str)] = &[
    (
        "Offitsial Dokumentatsiya",
        "https://doc.rust-lang.org/book/",
    ),
    ("O'zbek tilidagi varianti", "https://book.rust-lang.uz"),
    (
        "Rust by Example",
        "https://doc.rust-lang.org/rust-by-example/",
    ),
    ("Rustling", "https://github.com/rust-lang/rustlings"),
];

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    if !hooks::is_private(bot, msg).await.unwrap() {
        return Ok(());
    }

    bot.send_message(msg.chat.id, ROADMAP)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();

    for link in LINKS {
        keyboard.url(link.0, link.1);
        keyboard.row();
    }

    keyboard.get()
}
