import start from "./start.ts";
import help from "./help.ts";
import inline from "./inline.ts";
import { Bot } from "../deps.ts";
import about from "./about.ts";
import groups from "./groups.ts";
import playground from "./playground.ts";

export default async (bot: Bot) => {
  await bot
    .use(start)
    .use(help)
    .use(inline)
    .use(groups)
    .use(about)
    .use(playground);
};
