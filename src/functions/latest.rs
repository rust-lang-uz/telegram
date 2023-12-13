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
        "<b>Latest version is <a href=\"https://releases.rs/docs/{}\">\
        {}</a> released at </b> <code>{}</code> <b>by</b> <a href=\"{}\">\
        {}</a>.\
        \n\n\
        <b>In order to install latest update, execute</b> <code>rustup update</code> <b>on your terminal!</b>
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
    keyboard.url("More Information", release.html_url.as_str())
}
