import { config } from "../deps.ts";

const inits = () => {
  if (Deno.env.get("MODE") === "NOFS") {
    return Deno.env.toObject();
  } else {
    return config();
  }
};

const dots = inits();

export default dots;
