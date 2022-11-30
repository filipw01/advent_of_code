//Find possible, after an Elve remembered something
function hasIncreasingDigits(code) {
  let ruleBroken = false;
  String(code)
    .split("")
    .reduce((prevDigit, currentDigit) => {
      if (prevDigit > currentDigit) {
        ruleBroken = true;
      }
      return currentDigit;
    });
  return !ruleBroken;
}

function hasDoubleDigit(code) {
  let ruleFulfilled = false;
  for (const digit of String(code)) {
    const regex = new RegExp(digit, "g");
    if (String(code).match(regex).length === 2) {
      ruleFulfilled = true;
    }
  }
  return ruleFulfilled;
}

const minCode = 248345;
const maxCode = 746315;

const possibleCodes = [];

for (let code = minCode; code <= maxCode; code++) {
  if (hasDoubleDigit(code) && hasIncreasingDigits(code)) {
    possibleCodes.push(code);
  }
}

console.log(possibleCodes);
console.log(possibleCodes.length);
