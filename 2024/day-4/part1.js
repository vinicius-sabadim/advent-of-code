import { getLines } from "./utils";

const lines = getLines();
const grid = [];

lines.forEach((line, index) => {
  grid[index] = line.split("");
});

let sum = 0;

grid.forEach((line, lineIndex) => {
  line.forEach((letter, letterIndex) => {
    if (letter === "X") {
      const canGoForward = letterIndex < line.length - 3;
      const canGoBackwards = letterIndex > 2;
      const hasEnoughLinesBelow = lineIndex < grid.length - 3;
      const hasEnoughLinesAbove = lineIndex > 2;

      // horizontal forward
      if (
        canGoForward &&
        line[letterIndex + 1] === "M" &&
        line[letterIndex + 2] === "A" &&
        line[letterIndex + 3] === "S"
      ) {
        sum++;
      }

      // horizontal backwards
      if (
        canGoBackwards &&
        line[letterIndex - 1] === "M" &&
        line[letterIndex - 2] === "A" &&
        line[letterIndex - 3] === "S"
      ) {
        sum++;
      }

      // vertical down
      if (
        hasEnoughLinesBelow &&
        grid[lineIndex + 1][letterIndex] === "M" &&
        grid[lineIndex + 2][letterIndex] === "A" &&
        grid[lineIndex + 3][letterIndex] === "S"
      ) {
        sum++;
      }

      // vertical up
      if (
        hasEnoughLinesAbove &&
        grid[lineIndex - 1][letterIndex] === "M" &&
        grid[lineIndex - 2][letterIndex] === "A" &&
        grid[lineIndex - 3][letterIndex] === "S"
      ) {
        sum++;
      }

      // diagonal down forward
      if (
        canGoForward &&
        hasEnoughLinesBelow &&
        grid[lineIndex + 1][letterIndex + 1] === "M" &&
        grid[lineIndex + 2][letterIndex + 2] === "A" &&
        grid[lineIndex + 3][letterIndex + 3] === "S"
      ) {
        sum++;
      }

      // diagonal down backwards
      if (
        canGoBackwards &&
        hasEnoughLinesBelow &&
        grid[lineIndex + 1][letterIndex - 1] === "M" &&
        grid[lineIndex + 2][letterIndex - 2] === "A" &&
        grid[lineIndex + 3][letterIndex - 3] === "S"
      ) {
        sum++;
      }

      // diagonal up forward
      if (
        canGoForward &&
        hasEnoughLinesAbove &&
        grid[lineIndex - 1][letterIndex + 1] === "M" &&
        grid[lineIndex - 2][letterIndex + 2] === "A" &&
        grid[lineIndex - 3][letterIndex + 3] === "S"
      ) {
        sum++;
      }

      // diagonal up backwards
      if (
        canGoBackwards &&
        hasEnoughLinesAbove &&
        grid[lineIndex - 1][letterIndex - 1] === "M" &&
        grid[lineIndex - 2][letterIndex - 2] === "A" &&
        grid[lineIndex - 3][letterIndex - 3] === "S"
      ) {
        sum++;
      }
    }
  });
});

console.log(sum);
