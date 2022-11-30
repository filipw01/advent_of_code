//Find crossing wires by another criteria
import fs from "fs";

const file = fs.readFileSync("day3.txt", "utf-8");
const [firstWireInput, secondWireInput] = file.split("\n");

const firstWirePath = firstWireInput.split(",");
const secondWirePath = secondWireInput.split(",");

const map = {};
let currentPositionX = 0;
let currentPositionY = 0;
let stepsFromStart = 0;

const stepsFromStartCombined = [];

function drawLine(direction, steps, isFirstWire) {
  switch (direction) {
    case "U":
      while (steps > 0) {
        stepsFromStart++;
        if (isFirstWire) {
          map[`${currentPositionX}:${++currentPositionY}`] = stepsFromStart;
        } else {
          if (map[`${currentPositionX}:${++currentPositionY}`] !== undefined) {
            stepsFromStartCombined.push(
              map[`${currentPositionX}:${currentPositionY}`] + stepsFromStart
            );
          }
        }
        steps--;
      }
      break;

    case "D":
      while (steps > 0) {
        stepsFromStart++;
        if (isFirstWire) {
          map[`${currentPositionX}:${--currentPositionY}`] = stepsFromStart;
        } else {
          if (map[`${currentPositionX}:${--currentPositionY}`] !== undefined) {
            stepsFromStartCombined.push(
              map[`${currentPositionX}:${currentPositionY}`] + stepsFromStart
            );
          }
        }
        steps--;
      }
      break;

    case "R":
      while (steps > 0) {
        stepsFromStart++;
        if (isFirstWire) {
          map[`${++currentPositionX}:${currentPositionY}`] = stepsFromStart;
        } else {
          if (map[`${++currentPositionX}:${currentPositionY}`] !== undefined) {
            stepsFromStartCombined.push(
              map[`${currentPositionX}:${currentPositionY}`] + stepsFromStart
            );
          }
        }
        steps--;
      }
      break;

    case "L":
      while (steps > 0) {
        stepsFromStart++;
        if (isFirstWire) {
          map[`${--currentPositionX}:${currentPositionY}`] = stepsFromStart;
        } else {
          if (map[`${--currentPositionX}:${currentPositionY}`] !== undefined) {
            stepsFromStartCombined.push(
              map[`${currentPositionX}:${currentPositionY}`] + stepsFromStart
            );
          }
        }
        steps--;
      }
      break;

    default:
      break;
  }
}

for (const lineData of firstWirePath) {
  drawLine(lineData[0], lineData.slice(1), true);
}
currentPositionX = 0;
currentPositionY = 0;
stepsFromStart = 0;
for (const lineData of secondWirePath) {
  drawLine(lineData[0], Number(lineData.slice(1)), false);
}

console.log(stepsFromStartCombined);
const minStepsFromStart = stepsFromStartCombined.reduce(
  (currentMin, nextValue) => (currentMin < nextValue ? currentMin : nextValue)
);

console.log(minStepsFromStart);
