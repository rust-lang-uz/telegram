import { Composer, Context, InlineKeyboard } from "../deps.ts";

const composer = new Composer();

export const message = `<b>List of available commands:</b>` +
  `\n` +
  `\n` +
  `/help - <code>show this message</code>` +
  `\n` +
  `/about - <code>briefly about this bot</code>` +
  `\n` +
  `/last - <code>get short info about latest version</code>` +
  `\n` +
  `/version - <code>get information about specific version</code>` +
  `\n` +
  `/group - <code>rust communities</code>` +
  `\n` +
  `/run &lt;code&gt; - <code>run code and show stout & sterr</code>` +
  `\n` +
  `\n` +
  `<b>Additionally, you can use inline mode to search packages from crates.io. Press the button below to get started!</b>`;

export const keyboard = new InlineKeyboard().switchInlineCurrent(
  "Start searching crates!",
  "rand",
);

composer.command("help", async (ctx: Context): Promise<void> => {
  await ctx.reply(message, {
    parse_mode: "HTML",
    reply_markup: keyboard,
  });
});

export default composer;
