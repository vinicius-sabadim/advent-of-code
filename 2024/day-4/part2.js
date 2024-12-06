import { getLines } from "./utils";

const lines = getLines();
const grid = [];

lines.forEach((line, index) => {
  grid[index] = line.split("");
});

let sum = 0;

grid.forEach((line, lineIndex) => {
  line.forEach((letter, letterIndex) => {
    if (letter === "A") {
      const canGoForward = letterIndex < line.length - 1;
      const canGoBackwards = letterIndex > 0;
      const hasEnoughLinesBelow = lineIndex < grid.length - 1;
      const hasEnoughLinesAbove = lineIndex > 0;

      if (!canGoForward) return;
      if (!canGoBackwards) return;
      if (!hasEnoughLinesBelow) return;
      if (!hasEnoughLinesAbove) return;

      if (
        ((grid[lineIndex - 1][letterIndex - 1] === "M" &&
          grid[lineIndex + 1][letterIndex + 1] === "S") ||
          (grid[lineIndex - 1][letterIndex - 1] === "S" &&
            grid[lineIndex + 1][letterIndex + 1] === "M")) &&
        ((grid[lineIndex - 1][letterIndex + 1] === "M" &&
          grid[lineIndex + 1][letterIndex - 1] === "S") ||
          (grid[lineIndex - 1][letterIndex + 1] === "S" &&
            grid[lineIndex + 1][letterIndex - 1] === "M"))
      ) {
        sum++;
      }
    }
  });
});

console.log(sum);
