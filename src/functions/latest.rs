use crate::utils::{github::GitHub, keyboard::Keyboard, message::Rustina};
use octocrab::models::repos::Release;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

pub async fn command(bot: &Bot, github: GitHub, msg: &Message) -> ResponseResult<()> {
    let latest = github.get_latest().await.unwrap();

    bot.send_message_tf(msg.chat.id, view(&latest), msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard(&latest))
        .await?;

    Ok(())
}

pub fn view(release: &Release) -> String {
    format!(
        "<b>Hozirgi eng oxirgi versiya bu <a href=\"https://releases.rs/docs/{}\">\
        {}</a> va ushbu reliz </b> <code>{}</code> da e'lon qilingan <a href=\"{}\">\
        {}</a> tomonidan.\
        \n\n\
        ",
        release.tag_name,
        release.tag_name,
        release.published_at.unwrap().date_naive(),
        release.author.html_url,
        release.author.login,
    )
}

pub fn keyboard(release: &Release) -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard.url("Ko'proq ma'lumotlar", release.html_url.as_str())
}
