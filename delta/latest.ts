import { Composer, Context, InlineKeyboard } from "../deps.ts";
import type { Release } from "../types/Github.d.ts";
import { last } from "../utils/generator.ts";
import hecker from "../utils/checker.ts";

const composer = new Composer();

export const message = async (data: Release) =>
  // make a message about the release date of the new release
  `<b>Latest version is <a href="${
    (await hecker(data.tag_name, data.body)).url
  }">${data.tag_name}</a> released at</b> <code>${
    new Date(data.published_at).toDateString()
  }</code> by <a href="${data.author.html_url}">${data.author.login}</a>` +
  `\n` +
  `\n` +
  `<b>In order to install latest update, execute</b> <code>rustup update</code> <b>on your terminal!</b>`;

export const keyboard = (data: Release) =>
  new InlineKeyboard().url("More Information", data.html_url);

composer.command("last", async (ctx: Context): Promise<void> => {
  const req = await last();
  await ctx.reply(await message(req), {
    parse_mode: "HTML",
    reply_markup: keyboard(req),
  });
});

export default composer;
