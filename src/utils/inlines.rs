use super::keyboard::Keyboard;
use crates_io_api::Crate;
use teloxide::types::*;

pub static NO_INPUT: &str = r#"
<b>Salom foydalanuvchi!</b>

Siz inline rejim ishga tushurdingiz. Ushbu qulaylik yordamida siz crates.io Rust dasturlash tili paketlar registridan web sahifani ishlatmasdan turib telegram o'zida kerakli paketlarni qidirishingiz mumkin! Ushbu qulaylik yozish uchun o'zimizning <a href="https://github.com/rust-lang-uz/crates.ts">API SDK</a> ishlatildi.

Qidirishni boshlash uchun: 
<code>@rustaceanbot &lt;nomi&gt;</code>
"#;

pub fn view_generate(c: &Crate) -> String {
    let mut result = String::from("<b>🦀 Rust Telegram Cratelari 🦀</b>\n\n");

    result.push_str(&format!("📦 <b>Nomi:</b> {}\n", c.name));
    result.push_str(&format!(
        "🚨 <b>Oxirgi versiya:</b> <code>{}</code>\n",
        c.max_version
    ));
    result.push_str(&format!(
        "🎚 <b>Yuklab olingan:</b> yaqin orada: <code>{}</code> | hammasi: <code>{}</code>\n",
        c.recent_downloads.unwrap(),
        c.downloads
    ));
    result.push_str(&format!(
        "⌚️ <b>Yaratilgan:</b> <code>{}</code>\n",
        c.created_at.date_naive()
    ));
    result.push_str(&format!(
        "📡 <b>Yangilangan:</b> <code>{}</code>\n",
        c.updated_at.date_naive()
    ));
    result.push_str(&format!(
        "📰 <b>Ma'lumot:</b> <code>{}{}</code>\n\n",
        if c.description.clone().unwrap().len() > 100 {
            c.description
                .clone()
                .unwrap()
                .chars()
                .take(100)
                .collect::<String>()
        } else {
            c.description.clone().unwrap()
        },
        if c.description.clone().unwrap().len() > 100 {
            "..."
        } else {
            ""
        }
    ));
    result.push_str("🔌 <b>Cargo.toml fayliga qo'shib qo'ying:</b> \n");
    result.push_str(&format!(
        "<code>[dependencies]</code>\n<code>{} = \"{}\"</code>",
        c.name, c.max_version
    ));

    result
}

pub fn kb_generate(c: &Crate) -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();

    keyboard.url(
        "Crate",
        format!("https://crates.io/crates/{}", c.name).as_str(),
    );

    if c.homepage.is_some() {
        keyboard.url("Asosiy", &c.homepage.clone().unwrap());
        keyboard.row();
    }

    if c.documentation.is_some() {
        keyboard.url("Dokumentatsiya", &c.documentation.clone().unwrap());
        keyboard.row();
    }

    if c.repository.is_some() {
        keyboard.url("Repozitoriya", &c.repository.clone().unwrap());
        keyboard.row();
    }

    keyboard.get()
}

pub fn err_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard.switch_inline_current("Qayta urinib ko'ramizmi?", "rand")
}
