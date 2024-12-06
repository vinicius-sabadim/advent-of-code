import { getList, isReportSafe } from "./utils";

const list = getList();

const safeReports = list.reduce((acc, line) => {
  const report = line.split(" ").map(Number);

  // Already safe, no further actions
  if (isReportSafe(report)) {
    return acc + 1;
  }

  let safe = false;
  for (let i = 0; i < report.length; i++) {
    const reportWithRemovedLevel = report.filter((_, index) => index !== i);

    if (isReportSafe(reportWithRemovedLevel)) {
      safe = true;
      break;
    }
  }

  return safe ? acc + 1 : acc;
}, 0);

console.log(safeReports);
