// deno-lint-ignore-file no-explicit-any
import { Composer, Context } from "../deps.ts";
import encoder from "../utils/encoder.ts";
import playground from "../utils/playground.ts";

const composer = new Composer();

composer.hears(/^\/run(.*)/g, async (ctx: Context): Promise<any> => {
  if (ctx.message!.text!.trim().length <= 4) {
    return await ctx.reply(
      "<b>You should enter some rusty code bruh!</b>\n\n<b>For example:</b>\n" +
        '/run <code>fn main() {\n    println!("Hello, world!");\n}</code>',
      {
        parse_mode: "HTML",
      },
    );
  }

  const instance = await playground(ctx.message!.text!.slice(4));

  await ctx.reply(
    `<b>📃 Result : (${instance.success ? "✅ Success" : "❌ Failed"})</b> \n\n` +
      `<b>Output:</b>\n<pre>${
        encoder(instance.stdout)
      }</pre>\n<b>Terminal:</b>\n<pre>${encoder(instance.stderr)}</pre>`,
    {
      parse_mode: "HTML",
      reply_to_message_id: ctx.message!.message_id,
    },
  );
});

export default composer;
