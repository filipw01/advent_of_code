// Calculate required fuel
import fs from "fs";

const file = fs.readFileSync("day1.txt", "utf-8");
const arrayOfMass = file.split("\n");

const arrayOfFuel = arrayOfMass.map(fuel => {
  return Math.floor(fuel / 3) - 2;
});
console.log("Fuel for each module", arrayOfFuel);

const totalFuel = arrayOfFuel.reduce((acc, cur) => acc + cur);
console.log("Total fuel required", totalFuel);


