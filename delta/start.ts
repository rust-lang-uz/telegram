import { Composer, Context, InlineKeyboard } from "../deps.ts";

const composer = new Composer();

export const message: string =
  `<b>Hello fellow (rustacean|rustina|rust evangelist)!</b> \n` +
  `\n` +
  `Glad that you decided to give a try this bot. ` +
  `This bot helps you to do some rusty things without leaving telegram ` +
  `social messenger and interact with various APIs way more conveniently. `;

export const keyboard = new InlineKeyboard()
  .url("Rust", "https://rust-lang.org")
  .url("GitHub", "https://github.com/rust-lang")
  .row()
  .url("Our Team", "https://github.com/rust-lang-uz")
  .url("Rustacean", "https://rustacean.net");

composer.command("start", async (ctx: Context): Promise<void> => {
  await ctx.reply(message, {
    parse_mode: "HTML",
    reply_markup: keyboard,
  });
});

export default composer;
