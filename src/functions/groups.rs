use crate::utils::groups::{Group, Groups};
use orzklv::telegram::{keyboard::Keyboard, topic::Topics};
use teloxide::{payloads::EditMessageTextSetters, prelude::*, types::*};

static TEXT: &str = "<b>Telegramdagi Rust Hamjamiyatlari yoki Guruhlari:</b>\nAgar o'zingizni guruhingizni qo'shmoqchi bo'lsangiz, bizni <a href='https://github.com/rust-lang-uz/telegram/blob/main/data/communities.json'>community.json</a> ni yangilang!";

pub async fn command(bot: &Bot, msg: &Message, groups: &Groups) -> ResponseResult<()> {
    bot.send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard_list(groups, 1))
        .link_preview_options(LinkPreviewOptions {
            is_disabled: true,
            url: None,
            prefer_small_media: false,
            prefer_large_media: false,
            show_above_text: false,
        })
        .await?;

    Ok(())
}

pub async fn callback_list(
    bot: &Bot,
    q: &CallbackQuery,
    args: &[&str],
    groups: &Groups,
) -> ResponseResult<()> {
    if !args.is_empty() {
        let om = match q.message.clone() {
            Some(m) => m,
            None => return Ok(()),
        };

        if let Some(Message { id, chat, .. }) = om.regular_message() {
            bot.edit_message_text(chat.id, *id, TEXT)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard_list(groups, args[0].parse().unwrap_or(1)))
                .link_preview_options(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                })
                .await?;
        } else if let Some(id) = q.inline_message_id.clone() {
            bot.edit_message_text_inline(id, "Ebe hay, nimadir bo'ldi...")
                .await?;
        }
    }

    Ok(())
}

pub async fn callback_detail(bot: &Bot, q: &CallbackQuery, args: &[&str]) -> ResponseResult<()> {
    let groups: Groups = Groups::new();
    let find = groups.find_group(args[1..].join("_").to_string());

    if !args.is_empty() {
        let om = match q.message.clone() {
            Some(m) => m,
            None => return Ok(()),
        };

        if let Some(Message { id, chat, .. }) = om.regular_message() {
            bot.edit_message_text(chat.id, *id, view_detail(&find))
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard_detail(args[0].parse().unwrap_or(1), &find))
                .await?;
        } else if let Some(id) = q.inline_message_id.clone() {
            bot.edit_message_text_inline(id, "Ebe hay, nimadir bo'ldi...")
                .await?;
        }
    }

    Ok(())
}

pub fn view_detail(data: &Option<Group>) -> String {
    match data {
        Some(d) => {
            format!(
                "<b>{}</b>\n\n<i>{}</i>\n\n<b>Use the following buttons to get to the links:</b>",
                d.name, d.about
            )
        }
        None => "<b>Ushbu guruh mavjud emas!</b>".to_string(),
    }
}

pub fn keyboard_list(groups: &Groups, page: i32) -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();

    for group in groups.get_groups(page, 5) {
        keyboard.text(
            &group.name,
            &format!(
                "detail_{}_{}",
                page,
                group.telegram.clone().replace('@', "")
            ),
        );
        keyboard.row();
    }

    if !groups.get_groups(page + 1, 5).is_empty() {
        keyboard.text("Keyingi ➡️", &format!("group_{}", page + 1));
    }

    if page > 1 {
        keyboard.text("⬅️ Oldingi", &format!("group_{}", page - 1));
    }

    keyboard.get()
}

pub fn keyboard_detail(page: i32, data: &Option<Group>) -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();

    if let Some(group) = data {
        keyboard
            .url("Telegram", &format!("https://t.me/{}", group.telegram))
            .unwrap();

        if group.link.is_some() {
            keyboard.url("Web", &group.link.clone().unwrap()).unwrap();
        }

        keyboard.row();

        keyboard.text("🔙 Orqaga", &format!("group_{}", page));
    } else {
        keyboard.text("🔙 Orqaga", &format!("group_{}", page));
    }

    keyboard.get()
}
