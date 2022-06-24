import communities from "../communities.json" assert { type: "json" };

export default (page_number: number, page_size = 5) => {
  return communities.slice(
    (page_number - 1) * page_size,
    page_number * page_size,
  );
};
