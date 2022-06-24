// deno-lint-ignore-file no-explicit-any
import { Composer, Context } from "../deps.ts";

const composer = new Composer();

composer.hears(/^\/run(.*)/g, async (ctx: Context): Promise<any> => {
  if (ctx.message!.text!.trim().length <= 4) {
    return await ctx.reply("You should enter some rusty code bruh!", {
      parse_mode: "HTML",
    });
  }

  console.log(ctx.message!.text!.trim().length <= 4);

  const request = await fetch("https://play.rust-lang.org/execute", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      channel: "stable",
      mode: "debug",
      edition: "2021",
      crateType: "bin",
      tests: false,
      code: ctx.message!.text!.slice(4).trim(),
      backtrace: false,
    }),
  });

  const response = await request.json();

  await ctx.reply(
    `<b>📃 Result : (${response.success ? "✅ Success" : "❌ Failed"})</b> \n\n` +
      `<b>Output:</b>\n<pre>${
        decodeURI(response.stdout)
      }</pre>\n<b>Terminal:</b>\n<pre>${decodeURI(response.stderr)}</pre>`,
    {
      parse_mode: "HTML",
      reply_to_message_id: ctx.message!.message_id,
    },
  );
});

export default composer;
