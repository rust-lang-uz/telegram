// deno-lint-ignore-file no-explicit-any
import { Composer, Context, InputFile } from "../deps.ts";
import encoder from "../utils/encoder.ts";
import playground from "../utils/playground.ts";
import env from "../utils/config.ts";

const composer = new Composer();

composer.hears(/^\/(run|run@rustaceanbot)(.*)/g, async (ctx: Context): Promise<any> => {
  const code = ctx.match!;
  console.log(code);
  console.log(Deno.env.get("EDITOR"));

  if (ctx.message!.text!.trim().length <= 4) {
    return await ctx.reply(
      "<b>You should enter some rusty code or send me rusty code file on PM bruh!</b>\n\n<b>For example:</b>\n" +
        '/run <code>fn main() {\n    println!("Hello, world!");\n}</code>\n\n' +
        "<b>Btw, if you gonna send me some obscure loop shit, I'll simply ignore it :)</b>",
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

composer.on("message:document", async (ctx: Context): Promise<any> => {
  if (ctx.chat!.type !== "private") return;
  if (!ctx.message!.document!.file_name!.endsWith(".rs")) {
    await ctx.reply(
      "<b>You should send me rusty code with UTF8 encode!</b>\n" +
        "Btw, if you gonna send me some obscure loop shit, I'll simply ignore it :)" +
        "\n\n<b>For example:</b>",
      {
        parse_mode: "HTML",
      },
    );

    await ctx.replyWithDocument(
      new InputFile({
        url:
          "https://raw.githubusercontent.com/rust-lang-uz/telegram/main/example.rs",
      }),
      {
        parse_mode: "HTML",
      },
    );

    return;
  }

  const request = await fetch(
    "https://api.telegram.org/file/bot" + env["TOKEN"] + "/" +
      (await ctx.getFile()).file_path,
  );
  const instance = await playground(await request.text());

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
