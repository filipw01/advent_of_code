// Calculate required fuel (including fuel for fuel)
import fs from "fs";

const file = fs.readFileSync("day1.txt", "utf-8");
const arrayOfMass = file.split("\n");

const arrayOfFuel = arrayOfMass.map(fuel => {
  let totalFuel = 0;
  let extraFuel = Number(Math.floor(fuel / 3) - 2);
  while (extraFuel > 0) {
    totalFuel += extraFuel;
    extraFuel = Math.floor(extraFuel / 3) - 2;
  }
  return totalFuel;
});
console.log("Fuel for each module", arrayOfFuel);

const totalFuel = arrayOfFuel.reduce((acc, cur) => acc + cur);
console.log("Total fuel required (including fuel for fuel)", totalFuel);
