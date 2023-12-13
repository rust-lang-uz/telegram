pub mod github;
pub mod groups;
pub mod inlines;
pub mod keyboard;
pub mod message;
pub mod resources;

pub fn cargo_like_log(title: &str, message: &str) {
    println!(
        "{}\x1b[1;32m{}\x1b[0m {} {}",
        " ".repeat(12 - title.len()),
        title,
        message,
        " ".repeat(8)
    );
}
