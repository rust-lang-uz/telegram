use crate::utils::{
    keyboard::Keyboard,
    message::Rustina,
    resources::{Resource, Resources},
};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = "<b>Rustga oid foydali materiallar:</b>\n\
Agar o'zingizdan material qo'shmoqchi bo'lsangiz, bizni \
<a href='https://github.com/rust-lang-uz/rustina/blob/main/data/source.json'>\
source.json</a> ni yangilang!";

pub async fn command(bot: &Bot, msg: &Message, resources: &Resources) -> ResponseResult<()> {
    let categories = resources.get_keys();

    bot.send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard_list(categories))
        .disable_web_page_preview(true)
        .await?;

    Ok(())
}

pub async fn callback_list(
    bot: &Bot,
    q: &CallbackQuery,
    resources: &Resources,
) -> ResponseResult<()> {
    let categories = resources.get_keys();

    if let Some(Message { id, chat, .. }) = q.message.clone() {
        bot.edit_message_text(chat.id, id, TEXT)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard_list(categories))
            .disable_web_page_preview(true)
            .await?;
    } else if let Some(id) = q.inline_message_id.clone() {
        bot.edit_message_text_inline(id, "Oopsie, something went wrong...")
            .await?;
    }

    Ok(())
}

pub async fn callback_category_list(
    bot: &Bot,
    q: &CallbackQuery,
    args: &Vec<&str>,
    resources: &Resources,
) -> ResponseResult<()> {
    let find = resources.get_materials(args.join("_").as_str()).unwrap();

    if !args.is_empty() {
        if let Some(Message { id, chat, .. }) = q.message.clone() {
            bot.edit_message_text(chat.id, id, view_category_list(&args.join(" ")))
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard_category_list(find, args.join("_")))
                .await?;
        } else if let Some(id) = q.inline_message_id.clone() {
            bot.edit_message_text_inline(id, "Oopsie, something went wrong...")
                .await?;
        }
    }

    Ok(())
}

pub async fn callback_material_detail(
    bot: &Bot,
    q: &CallbackQuery,
    args: &Vec<&str>,
    resources: &Resources,
) -> ResponseResult<()> {
    let find = resources
        .get_material(args[1..].join("_").as_str(), args[0].parse().unwrap())
        .unwrap();

    if !args.is_empty() {
        if let Some(Message { id, chat, .. }) = q.message.clone() {
            bot.edit_message_text(chat.id, id, view_material_detail(find))
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard_material_detail(find, args[1..].join("_")))
                .await?;
        } else if let Some(id) = q.inline_message_id.clone() {
            bot.edit_message_text_inline(id, "Oopsie, something went wrong...")
                .await?;
        }
    }

    Ok(())
}

pub fn view_category_list(category: &str) -> String {
    format!("<b>Siz hozirda {}{} kategoriyasi ichida turibsiz.</b>\nIltimos, keltirilgan materiallardan birini tanlang...", 
        &category[0..1].to_uppercase(), &category[1..].replace('_', " "))
}

pub fn view_material_detail(material: &Resource) -> String {
    format!(
        "<b>{}</b>\n\n<i>{}</i>\n\n<b>Ushbu pastdagi tugmacha orqali lavha ga o'tib oling:</b>",
        material.name, material.desc
    )
}

pub fn keyboard_list(categories: Vec<String>) -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();

    for category in categories {
        keyboard.text(
            &format!(
                "{}{}",
                &category[0..1].to_uppercase(),
                &category[1..].replace('_', " ")
            ),
            &format!("category_{}", category),
        );
        keyboard.row();
    }

    keyboard.get()
}

pub fn keyboard_category_list(material: &[Resource], category: String) -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();

    for (index, value) in material.iter().enumerate() {
        keyboard.text(
            &format!(
                "{}{}",
                &value.name[0..1].to_uppercase(),
                &value.name[1..].replace('_', " ")
            ),
            &format!("material_{}_{}", index, category),
        );
        keyboard.row();
    }

    keyboard.text("🔙 Orqaga", "useful");

    keyboard.get()
}

pub fn keyboard_material_detail(resource: &Resource, category: String) -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();

    keyboard.url("Web Sahifasi", &resource.link);
    keyboard.row();
    keyboard.text("🔙 Orqaga", &format!("category_{}", category));

    keyboard.get()
}
