import type { Release } from "../types/Github.d.ts";

export const finder = async (id: number) => {
  const request = await fetch(
    `https://api.github.com/repos/rust-lang/rust/releases/${id}`,
  );

  return await request.json() as Release;
};

export const pager = async (page: number, size = 5) => {
  const request = await fetch(
    `https://api.github.com/repos/rust-lang/rust/releases?per_page=${size}&page=${page}`,
  );

  return await request.json() as Release[];
};

export const last = async () => {
  const request = await fetch(
    `https://api.github.com/repos/rust-lang/rust/releases/latest`,
  );

  return await request.json() as Release;
};
