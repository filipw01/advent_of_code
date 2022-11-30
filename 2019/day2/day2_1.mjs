//Intcode computer
import fs from "fs";

const file = fs.readFileSync("day2.txt", "utf-8");
let input = file.split(",");
input[1] = 12;
input[2] = 2;
input = input.map(el => Number(el));
const inputCopy = [...input];

const parseToInstructions = input => {
  for (let i = 0; input[i] !== undefined; i += 4) {
    switch (input[i]) {
      case 1:
        input[input[i + 3]] = input[input[i + 1]] + input[input[i + 2]];
        break;

      case 2:
        input[input[i + 3]] = input[input[i + 1]] * input[input[i + 2]];
        break;

      case 99:
        return input;

      default:
        throw new Error(`Unknown instruction ${input[i]}`);
    }
  }
  return input;
};

const output = parseToInstructions(inputCopy);
console.log(input);
console.log(output);
