use super::keyboard::Keyboard;
use crates_io_api::Crate;
use teloxide::types::*;

pub static NO_INPUT: &str = r#"
<b>Hello user!</b>

You just tried inline mode. This feature helps you to search packages from crates.io using <a href="https://github.com/rust-lang-uz/crates.ts">API SDK</a> typescript library. In order to start searching, simply write: 

<code>@rustaceanbot &lt;name&gt;</code>
"#;

pub fn view_generate(c: &Crate) -> String {
    let mut result = String::from("<b>🦀 Rusty Telegram Crate 🦀</b>\n\n");

    result.push_str(&format!("📦 <b>Name:</b> {}\n", c.name));
    result.push_str(&format!(
        "🚨 <b>Last Version:</b> <code>{}</code>\n",
        c.max_version
    ));
    result.push_str(&format!(
        "🎚 <b>Downloads:</b> recent: <code>{}</code> | all: <code>{}</code>\n",
        c.recent_downloads.unwrap(),
        c.downloads
    ));
    result.push_str(&format!(
        "⌚️ <b>Created:</b> <code>{}</code>\n",
        c.created_at.date_naive()
    ));
    result.push_str(&format!(
        "📡 <b>Updated:</b> <code>{}</code>\n",
        c.updated_at.date_naive()
    ));
    result.push_str(&format!(
        "📰 <b>Description:</b> <code>{}{}</code>\n\n",
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
    result.push_str("🔌 <b>Add (in your Cargo.toml):</b> \n");
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
        keyboard.url("Homepage", &c.homepage.clone().unwrap());
        keyboard.row();
    }

    if c.documentation.is_some() {
        keyboard.url("Documentation", &c.documentation.clone().unwrap());
        keyboard.row();
    }

    if c.repository.is_some() {
        keyboard.url("Repository", &c.repository.clone().unwrap());
        keyboard.row();
    }

    keyboard.get()
}

pub fn err_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard.switch_inline_current("Shall we try again?", "rand")
}
