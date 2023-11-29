use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use url::Url;

#[derive(Clone)]
pub struct Keyboard {
    keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard::new()
    }
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keyboard: vec![vec![]],
        }
    }

    /// Add a text callback to keyboard
    pub fn text(&mut self, text: &str, callback: &str) -> InlineKeyboardMarkup {
        self.keyboard
            .last_mut()
            .unwrap()
            .push(InlineKeyboardButton::callback(text, callback));

        self.get()
    }

    /// Add an url button to keyboard
    pub fn url(&mut self, text: &str, url: &str) -> InlineKeyboardMarkup {
        let parsed_url = Url::parse(url).unwrap();

        self.keyboard
            .last_mut()
            .unwrap()
            .push(InlineKeyboardButton::url(text, parsed_url));

        self.get()
    }

    pub fn switch_inline_current(&mut self, text: &str, query: &str) -> InlineKeyboardMarkup {
        self.keyboard.last_mut().unwrap().push(
            InlineKeyboardButton::switch_inline_query_current_chat(text, query),
        );

        self.get()
    }

    /// Add next buttons from new line
    pub fn row(&mut self) -> InlineKeyboardMarkup {
        self.keyboard.push(vec![]);
        self.get()
    }

    /// Return the final result
    pub fn get(&self) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::new(self.keyboard.clone())
    }
}
