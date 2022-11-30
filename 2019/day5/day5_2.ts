// Intcode computer improved
const fs = require("fs");

const file = fs.readFileSync("day5.txt", "utf-8");
let input = file.split(",").map(el => Number(el));

function getWithMode(
  mode: number,
  input: Array<number>,
  index: number
): number {
  if (mode === 0) {
    return input[input[index]];
  }
  if (mode === 1) {
    return input[index];
  }
  throw new Error(`Unknown mode ${mode}`);
}

function setTable(input: Array<number>, index: number, value: number): number {
  return (input[input[index]] = value);
}

function parseInstruction(
  input: Array<number>,
  opcode: string,
  modes: Array<number>,
  baseParamIndex: number
): number {
  let numberOne: number;
  let numberTwo: number;
  switch (Number(opcode)) {
    case 1:
      numberOne = getWithMode(modes[0], input, baseParamIndex);
      numberTwo = getWithMode(modes[1], input, baseParamIndex + 1);
      setTable(input, baseParamIndex + 2, numberOne + numberTwo);
      return 4;

    case 2:
      numberOne = getWithMode(modes[0], input, baseParamIndex);
      numberTwo = getWithMode(modes[1], input, baseParamIndex + 1);
      setTable(input, baseParamIndex + 2, numberOne * numberTwo);
      return 4;

    case 3:
      setTable(input, baseParamIndex, 5);
      console.log("input: 5");
      return 2;

    case 4:
      console.log(`output ${input[input[baseParamIndex]]}`);
      return 2;

    case 5:
      numberOne = getWithMode(modes[0], input, baseParamIndex);
      numberTwo = getWithMode(modes[1], input, baseParamIndex + 1);
      if (numberOne !== 0) {
        console.log(numberTwo);
        return -baseParamIndex + 1 + numberTwo;
      }
      return 3;

    case 6:
      numberOne = getWithMode(modes[0], input, baseParamIndex);
      numberTwo = getWithMode(modes[1], input, baseParamIndex + 1);
      if (numberOne === 0) {
        console.log(numberTwo);
        return -baseParamIndex + 1 + numberTwo;
      }
      return 3;

    case 7:
      numberOne = getWithMode(modes[0], input, baseParamIndex);
      numberTwo = getWithMode(modes[1], input, baseParamIndex + 1);
      setTable(input, baseParamIndex + 2, numberOne < numberTwo ? 1 : 0);
      return 4;

    case 8:
      numberOne = getWithMode(modes[0], input, baseParamIndex);
      numberTwo = getWithMode(modes[1], input, baseParamIndex + 1);
      setTable(input, baseParamIndex + 2, numberOne === numberTwo ? 1 : 0);
      return 4;

    case 99:
      return null;

    default:
      throw new Error(`Unknown instruction ${opcode}`);
  }
}

const parseToInstructions = input => {
  let i = 0;
  while (input[i] !== undefined) {
    const opcode: string = ("0".repeat(2) + input[i]).slice(-2);
    const modes: Array<number> = (
      "0".repeat(5).slice(0, -input[i].toString().length) + input[i]
    )
      .split("")
      .slice(0, -2)
      .map(el => Number(el))
      .reverse();
    const parseResult = parseInstruction(input, opcode, modes, i + 1);

    if (parseResult !== null) {
      i += parseResult;
    } else {
      return 0;
    }
  }
  return input;
};
parseToInstructions(input);
