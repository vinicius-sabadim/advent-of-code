import { getList, isReportSafe } from "./utils";

const list = getList();

const safeReports = list.reduce((acc, line) => {
  const report = line.split(" ").map(Number);
  return isReportSafe(report) ? acc + 1 : acc;
}, 0);

console.log(safeReports);
