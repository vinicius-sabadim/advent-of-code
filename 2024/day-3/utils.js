const fs = require("fs");

export function getInput() {
  const data = fs.readFileSync("input.txt", "utf-8");

  return data;
}

export function calculateTheSumOfTheMultiInInstructions(instructions) {
  const multi_regex = /mul\(\d+,\s*\d+\)/g;
  const number_regex = /\d+/g;
  const matches = instructions.match(multi_regex);

  return matches.reduce((acc, item) => {
    const [num1, num2] = item.match(number_regex);

    return (acc += Number(num1) * Number(num2));
  }, 0);
}
