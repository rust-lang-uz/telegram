import { Composer, Context, InlineKeyboard } from "../deps.ts";
import { finder, pager as generator } from "../utils/generator.ts";

const composer = new Composer();
const ctxMenuText = "<b>List of Rust Programming LanguageVersion:</b>";

composer.command("version", async (ctx: Context): Promise<void> => {
  const keyboard = new InlineKeyboard();

  for (const release of await generator(1)) {
    keyboard.text(
      release.tag_name,
      `changelog_${1}_${release.id}`,
    ).row();
  }

  if ((await generator(2)).length > 0) {
    keyboard.text(`Next ➡️`, `version_2`);
  }

  await ctx.reply(ctxMenuText, {
    parse_mode: "HTML",
    reply_markup: keyboard,
    disable_web_page_preview: true,
  });
});

composer.callbackQuery(/^version_(\d+)$/, async (ctx: Context) => {
  const page = Number(ctx.match![1]);
  const keyboard = new InlineKeyboard();

  console.log(page);

  for (const release of await generator(page)) {
    keyboard.text(
      release.tag_name,
      `changelog_${page}_${release.id}`,
    ).row();
  }

  if (page > 1) {
    keyboard.text(`⬅️ Previous`, `version_${page - 1}`);
  }

  if ((await generator(page + 1)).length > 0) {
    keyboard.text(`Next ➡️`, `version_${page + 1}`);
  }

  await ctx.editMessageText(ctxMenuText, {
    parse_mode: "HTML",
    reply_markup: keyboard,
    disable_web_page_preview: true,
  });
});

composer.callbackQuery(/^changelog_(\d+)_(\d+)$/, async (ctx: Context) => {
  const keyboard = new InlineKeyboard();
  const page = Number(ctx.match![1]);
  const data = await finder(Number(ctx.match![2]));

  keyboard.url(
    `Read at GitHub`,
    data.html_url,
  );

  keyboard.row().text(`🔙 Back`, `version_${page}`);

  await ctx.editMessageText(
    `<b>${data.name}</b>` +
      `\n` +
      `\n` +
      `<b>Created at:</b>${new Date(data.created_at).toDateString()}` +
      `\n` +
      `<b>Published at:</b>${new Date(data.published_at).toDateString()}` +
      `\n` +
      `<i>Telegraph Content Integration cuming soon!</i>` +
      `\n` +
      `\n` +
      `<b>Use the following buttons to get to the links:</b>`,
    {
      parse_mode: "HTML",
      reply_markup: keyboard,
      disable_web_page_preview: true,
    },
  );
});

export default composer;
