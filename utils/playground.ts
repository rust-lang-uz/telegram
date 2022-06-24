export interface Compiler {
  success: boolean;
  stdout: string;
  stderr: string;
}

export default async (code: string): Promise<Compiler> => {
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
      code: code.trim(),
      backtrace: false,
    }),
  });

  return await request.json() as Compiler;
};
