import { Composer, Context, InlineKeyboard } from "../deps.ts";
import type { Release } from "../types/Github.d.ts";
import { last } from "../utils/generator.ts";

const composer = new Composer();

export const message = (data: Release) =>
  // make a message about the release date of the new release
  `Latest version is <b>${data.tag_name}</b> released at <code>${
    new Date(data.published_at).toDateString()
  }</code> by <a href="https://github.com/${data.author.login}">${data.author.login}</a>`;

export const keyboard = (data: Release) =>
  new InlineKeyboard().url("More Information", data.html_url);

composer.command("last", async (ctx: Context): Promise<void> => {
  const req = await last();
  await ctx.reply(message(req), {
    disable_web_page_preview: true,
    parse_mode: "HTML",
    reply_markup: keyboard(req),
  });
});

export default composer;
