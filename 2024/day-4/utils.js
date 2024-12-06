const fs = require("fs");

export function getLines() {
  const data = fs.readFileSync("input.txt", "utf-8");

  return data.split("\n");
}
