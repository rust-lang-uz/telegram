// deno-lint-ignore-file no-explicit-any
import { api, Composer, Context, InlineKeyboard } from "../deps.ts";

const composer = new Composer();

composer.inlineQuery(/(.*)/ig, async (ctx: Context): Promise<any> => {
  if (ctx.inlineQuery?.query) {
    const request = await api.search(ctx.inlineQuery?.query, 50);

    if (request.meta.total === 0) {
      return await ctx.answerInlineQuery([{
        type: "article",
        id: "404",
        title: "Error 404!",
        description: `No results found for ${ctx.inlineQuery?.query}!`,
        reply_markup: new InlineKeyboard().switchInlineCurrent(
          "Shall we try again?",
          "rand",
        ),
        input_message_content: {
          message_text:
            `<b>"${ctx.inlineQuery?.query}" did not match any packages!</b>` +
            `\n` +
            `Please, check for typos and try again!.`,
          parse_mode: "HTML",
        },
      }]);
    }

    return await ctx.answerInlineQuery(
      request.crates.map((item) => ({
        type: "article",
        id: crypto.randomUUID(),
        title: item.name,
        url: `https://crates.io/crates/${item.id}`,
        description: item.description,
        reply_markup: (() => {
          const keyboard = new InlineKeyboard();
          keyboard.url(`Crate`, `https://crates.io/crates/${item.name}`);
          if (item.homepage) keyboard.url(`Homepage`, item.homepage).row();
          if (item.documentation) {
            keyboard.url(`Documentation`, item.documentation).row();
          }
          if (item.repository) keyboard.url(`Repository`, item.repository);
          return keyboard;
        })(),
        input_message_content: {
          message_text: `<b>🦀 Rusty Telegram Crate 🦀</b>\n\n` +
            `📦 <b>Name:</b> ${item.name}` +
            `\n` +
            `🚨 <b>Last Version:</b> <code>${item.newest_version}</code> \n` +
            `⌚️ <b>Created:</b> <code>${
              new Date(item.created_at).toLocaleString()
            }</code> \n` +
            `📰 <b>Description:</b> <code>${
              (item.description.replaceAll(/(<|>|`)/ig, "")).substring(0, 150)
            }${item.description.length > 30 ? "..." : ""}</code> \n\n` +
            `🔌 <b>Add (in your Cargo.toml):</b> \n` +
            `<code>[dependencies]</code>\n<code>${item.name} = "${item.max_stable_version}"</code>`,
          parse_mode: "HTML",
        },
      })),
      { cache_time: 1 },
    );
  }

  if (!ctx.inlineQuery?.query) {
    return await ctx.answerInlineQuery([{
      type: "article",
      id: "101",
      title: "Start typing!",
      description: "Write the name of package you would like to search!",
      reply_markup: new InlineKeyboard().switchInlineCurrent(
        "Shall we try again?",
        "rand",
      ),
      input_message_content: {
        message_text: `<b>Hello user!</b>` +
          `\n` +
          `You just tried inline mode. This feature helps you to search packages ` +
          `from crates.io using <a href="https://github.com/rust-lang-uz/crates.ts">API SDK</a> typescript library. ` +
          `In order to start searching, simply write: ` +
          `\n` +
          `<code>@rustaceanbot &lt;name&gt;</code>`,
        parse_mode: "HTML",
      },
    }]);
  }
});

export default composer;
