const fs = require("fs");

export function getLists() {
  const data = fs.readFileSync("input.txt", "utf-8");

  const lines = data.split("\n");

  const list1 = [];
  const list2 = [];

  lines.forEach((line) => {
    const [value1, value2] = line.split(" ");
    list1.push(Number(value1));
    list2.push(Number(value2));
  });

  return { list1, list2 };
}

export function getAmountList(list) {
  return list.reduce((acc, value) => {
    if (acc[value]) {
      return {
        ...acc,
        [value]: acc[value] + 1,
      };
    }

    return {
      ...acc,
      [value]: 1,
    };
  }, {});
}
