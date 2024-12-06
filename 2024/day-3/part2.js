import { calculateTheSumOfTheMultiInInstructions, getInput } from "./utils";

const instructions = getInput();

let sum = 0;
let remaining = instructions;
let checkingFor = "dont";

while (remaining.length > 0) {
  if (checkingFor === "dont") {
    const indexOfNextDont = remaining.indexOf("don't()");

    // There is no more don't instruction
    if (indexOfNextDont === -1) {
      sum += calculateTheSumOfTheMultiInInstructions(remaining);
      break;
    } else {
      const instructionToCheck = remaining.substring(0, indexOfNextDont);
      sum += calculateTheSumOfTheMultiInInstructions(instructionToCheck);

      remaining = remaining.substring(indexOfNextDont);
      checkingFor = "do";
    }
  } else {
    const indexOfNextDo = remaining.indexOf("do()");

    // There is no more do instruction
    if (indexOfNextDo === -1) {
      break;
    } else {
      remaining = remaining.substring(indexOfNextDo);
      checkingFor = "dont";
    }
  }
}

console.log(sum);
