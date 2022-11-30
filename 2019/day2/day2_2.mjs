//Intcode computer 2
import fs from "fs";

const file = fs.readFileSync("day2.txt", "utf-8");
let input = file.split(",");

input = input.map(el => Number(el));

const parseToInstructions = (input, noun, verb) => {
  const inputCopy = [...input];
  inputCopy[1] = noun;
  inputCopy[2] = verb;
  for (let i = 0; inputCopy[i] !== undefined; i += 4) {
    switch (inputCopy[i]) {
      case 1:
        inputCopy[inputCopy[i + 3]] =
          inputCopy[inputCopy[i + 1]] + inputCopy[inputCopy[i + 2]];
        break;

      case 2:
        inputCopy[inputCopy[i + 3]] =
          inputCopy[inputCopy[i + 1]] * inputCopy[inputCopy[i + 2]];
        break;

      case 99:
        if (inputCopy[0] === 19690720) {
          console.log(inputCopy);
          console.log("Noun:", noun, "Verb:", verb);
        }
        return inputCopy;

      default:
        throw new Error(`Unknown instruction ${inputCopy[i]}`);
    }
  }
  return inputCopy;
};

let noun = 0;
let verb = 0;

while (true) {
  parseToInstructions(input, noun++, verb);
  verb = 0;
  while (noun >= verb) {
    parseToInstructions(input, noun, verb++);
  }
}
