//Find possible codes
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

function hasRepeatedDigit(code) {
  let ruleFulfilled = false;
  String(code)
    .split("")
    .reduce((prevDigit, currentDigit) => {
      if (prevDigit === currentDigit) {
        ruleFulfilled = true;
      }
      return currentDigit
    });
  return ruleFulfilled;
}

const minCode = 248345;
const maxCode = 746315;

const possibleCodes = [];

for (let code = minCode; code <= maxCode; code++) {
  if (hasRepeatedDigit(code) && hasIncreasingDigits(code)) {
    possibleCodes.push(code);
  }
}

console.log(possibleCodes);
console.log(possibleCodes.length);
