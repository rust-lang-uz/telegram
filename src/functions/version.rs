use crate::utils::{github::GitHub, keyboard::Keyboard, message::Rustina};
use octocrab::models::repos::Release;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = "<b>List of rust programming language versions:</b>";

pub async fn command(bot: &Bot, github: GitHub, msg: &Message) -> ResponseResult<()> {
    let versions = github.get_list(1).await.unwrap();
    let next_page = github.get_list(2).await.unwrap();

    bot.send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard_list(1, versions, Some(next_page)))
        .await?;

    Ok(())
}

pub async fn callback_list(
    bot: &Bot,
    q: &CallbackQuery,
    args: &Vec<&str>,
    github: GitHub,
) -> ResponseResult<()> {
    let page = args[0].parse::<u32>().unwrap();
    let versions: Vec<Release> = github.get_list(page).await.unwrap();
    let next_page = github.get_list(page + 1).await.unwrap();

    if !args.is_empty() {
        if let Some(Message { id, chat, .. }) = q.message.clone() {
            bot.edit_message_text(chat.id, id, TEXT)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard_list(page, versions, Some(next_page)))
                .await?;
        } else if let Some(id) = q.inline_message_id.clone() {
            bot.edit_message_text_inline(id, "Oopsie, something went wrong...")
                .await?;
        }
    }

    Ok(())
}

pub async fn callback_detail(
    bot: &Bot,
    q: &CallbackQuery,
    args: &Vec<&str>,
    github: GitHub,
) -> ResponseResult<()> {
    let page = args[0].parse::<u32>().unwrap();
    let version: Release = github.get_detail(args[1]).await.unwrap();

    if !args.is_empty() {
        if let Some(Message { id, chat, .. }) = q.message.clone() {
            bot.edit_message_text(chat.id, id, view_detail(&version))
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard_detail(page, version))
                .await?;
        } else if let Some(id) = q.inline_message_id.clone() {
            bot.edit_message_text_inline(id, "Oopsie, something went wrong...")
                .await?;
        }
    }

    Ok(())
}

pub fn view_detail(release: &Release) -> String {
    format!(
        "<b><a href=\"{}\">{}</a></b>\n\n\
        <b>Created at:</b> {}\n\
        <b>Published at:</b> {}\n\
        <b>Install:</b> <code>rustup install {}</code>\n\n\
        <b>Use \"Instant view\" or the following buttons to get more info:</b>",
        release.html_url,
        release.name.clone().unwrap(),
        release.created_at.unwrap().format("%d.%m.%Y"),
        release.published_at.unwrap().format("%d.%m.%Y"),
        release.tag_name
    )
}

pub fn keyboard_list(
    page: u32,
    releases: Vec<Release>,
    next_page: Option<Vec<Release>>,
) -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();

    for release in releases {
        keyboard.text(
            &release.tag_name,
            &format!("changelog_{}_{}", page, release.tag_name),
        );
        keyboard.row();
    }

    if page > 1 {
        keyboard.text("⬅️ Previous", &format!("version_{}", page - 1));
    }

    if next_page.is_some() && !next_page.unwrap().is_empty() {
        keyboard.text("Next ➡️", &format!("version_{}", page + 1));
    }

    keyboard.get()
}

pub fn keyboard_detail(page: u32, release: Release) -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();

    keyboard.url("📝 Read at GitHub", release.html_url.as_str());
    keyboard.row();
    keyboard.text("🔙 Back", &format!("version_{}", page));

    keyboard.get()
}
