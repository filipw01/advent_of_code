//Find crossing wires
import fs from "fs";

const file = fs.readFileSync("day3.txt", "utf-8");
const [firstWireInput, secondWireInput] = file.split("\n");

const firstWirePath = firstWireInput.split(",");
const secondWirePath = secondWireInput.split(",");

const map = {};
let currentPositionX = 0;
let currentPositionY = 0;

const crossingCoordinates = [];

function drawLine(direction, steps, isFirstWire) {
  switch (direction) {
    case "U":
      while (steps > 0) {
        if (isFirstWire) {
          map[`${currentPositionX}:${++currentPositionY}`] = 1;
        } else {
          if (map[`${currentPositionX}:${++currentPositionY}`] === 1) {
            crossingCoordinates.push([currentPositionX, currentPositionY]);
          }
        }
        steps--;
      }
      break;

    case "D":
      while (steps > 0) {
        if (isFirstWire) {
          map[`${currentPositionX}:${--currentPositionY}`] = 1;
        } else {
          if (map[`${currentPositionX}:${--currentPositionY}`] === 1) {
            crossingCoordinates.push([currentPositionX, currentPositionY]);
          }
        }
        steps--;
      }
      break;

    case "R":
      while (steps > 0) {
        if (isFirstWire) {
          map[`${++currentPositionX}:${currentPositionY}`] = 1;
        } else {
          if (map[`${++currentPositionX}:${currentPositionY}`] === 1) {
            crossingCoordinates.push([currentPositionX, currentPositionY]);
          }
        }
        steps--;
      }
      break;

    case "L":
      while (steps > 0) {
        if (isFirstWire) {
          map[`${--currentPositionX}:${currentPositionY}`] = 1;
        } else {
          if (map[`${--currentPositionX}:${currentPositionY}`] === 1) {
            crossingCoordinates.push([currentPositionX, currentPositionY]);
          }
        }
        steps--;
      }
      break;

    default:
      break;
  }
}

function calculateManhattanDistance(coordinates) {
  const [x, y] = coordinates;
  return Math.abs(x) + Math.abs(y);
}

for (const lineData of firstWirePath) {
  drawLine(lineData[0], lineData.slice(1), true);
}
currentPositionX = 0;
currentPositionY = 0;
for (const lineData of secondWirePath) {
  drawLine(lineData[0], Number(lineData.slice(1)), false);
}

console.log(crossingCoordinates);
const longestDistance = crossingCoordinates
  .map(coordinate => calculateManhattanDistance(coordinate))
  .reduce((currentMin, nextValue) =>
    currentMin < nextValue ? currentMin : nextValue
  );

console.log(longestDistance);
