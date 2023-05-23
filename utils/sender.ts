// deno-lint-ignore-file no-explicit-any
import { Context, InlineKeyboard } from "../deps.ts";

/**
 * Reply to message api but with topics support
 * @param ctx Context from Grammy.js middleware
 * @param message The message you want to send
 * @param buttons InlineKeyboard button to attach to the message
 * @param configs Other configs to pass to the api
 */
export const reply = async (
  ctx: Context,
  message: string,
  buttons?: InlineKeyboard,
  configs?: { [key: string]: any }
): Promise<any> => {
  const config: { [key: string]: any } = {
    parse_mode: "HTML",
    ...configs,
  };

  if (ctx.message!.is_topic_message) {
    config["message_thread_id"] = ctx.message!.message_thread_id;
  }

  if (buttons) {
    config["reply_markup"] = buttons;
  }

  return await ctx.reply(message, config);
};