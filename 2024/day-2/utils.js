const fs = require("fs");

export function getList() {
  const data = fs.readFileSync("input.txt", "utf-8");

  return data.split("\n");
}

export function isReportSafe(report) {
  let safe = true;
  const listType = report[0] > report[1] ? "decreasing" : "increasing";

  for (let i = 0; i < report.length - 1; i++) {
    // Same number
    if (report[i] === report[i + 1]) {
      safe = false;
      break;
    }

    // Difference is bigger than 3
    if (Math.abs(report[i] - report[i + 1]) > 3) {
      safe = false;
      break;
    }

    // Levels are not decreasing
    if (listType === "decreasing" && report[i] < report[i + 1]) {
      safe = false;
      break;
    }

    // Levels are not increasing
    if (listType === "increasing" && report[i] > report[i + 1]) {
      safe = false;
      break;
    }
  }

  return safe;
}
