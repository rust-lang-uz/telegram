import { Composer, Context, InlineKeyboard } from "../deps.ts";

const composer = new Composer();

export const message =
  `<b>Hello fellow (rustacean|rustina|evangelist)!</b> \n` +
  `\n` +
  `This is a telegram bot created by a rustacean to help ` +
  `people interact with various Rust API way more conveniently.`;

export const keyboard = new InlineKeyboard().url(
  `Source Code`,
  `https://github.com/rust-lang-uz/telegram`,
).url(
  `Author`,
  `https://github.com/uwussimo`,
);

composer.command("about", async (ctx: Context): Promise<void> => {
  await ctx.reply(message, {
    parse_mode: "HTML",
    reply_markup: keyboard,
  });
});

export default composer;
