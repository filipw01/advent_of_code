// Calculate orbits map checksum
const fs = require("fs");

const file = fs.readFileSync("day6.txt", "utf-8");
let input = file.split("\n").map(el => el.split(")"));

const map: Object = {};

//create map object from file
for (const orbit of input) {
  const [orbited, orbiting] = orbit;

  map[orbited] = {
    directlyOrbitedBy: {
      ...map[orbited]?.directlyOrbitedBy,
      [orbiting]: true
    }
  };
}

// insert childName planet from root to the planet it's orbiting
function deepInsertChild(
  rootMap: Object,
  map: Object,
  childName: string
): 0 | 1 {
  if (Object.keys(map).length === 0) return 0;
  for (const key in map) {
    if (key === childName || map[key].directlyOrbitedBy === undefined) continue;
    if (map[key].directlyOrbitedBy[childName]) {
      if (map[key] == undefined) continue;
      map[key].directlyOrbitedBy[childName] = rootMap[childName];
      delete rootMap[childName];
      console.log(`${childName} moved`);
      return 1;
    }
    if (deepInsertChild(rootMap, map[key].directlyOrbitedBy, childName) == 1) {
      return 1;
    }
  }
  return 0;
}

// reduce the map to the graph until there is only one element that isn't orbiting anything
while (Object.keys(map).length !== 1) {
  for (const key in map) {
    if (map.hasOwnProperty(key)) {
      deepInsertChild(map, map, key);
    }
  }
}

console.log(map);

// sum direct and relative orbits form the graph
function sumOrbits(graphFromCOM: Object): number {
  let totalOrbits: number = 0;
  let orbitsAway: number = 0;
  let graphToScan: Object = graphFromCOM;
  let furtherOrbitGraphs = {};
  while (true) {
    totalOrbits += Object.keys(graphToScan).length * orbitsAway;

    for (const planetName in graphToScan) {
      if (graphToScan.hasOwnProperty(planetName)) {
        const childOrbitGraphs = graphToScan[planetName].directlyOrbitedBy;
        furtherOrbitGraphs = { ...furtherOrbitGraphs, ...childOrbitGraphs };
      }
    }
    if (Object.keys(graphToScan).length === 0) return totalOrbits;
    graphToScan = furtherOrbitGraphs;
    furtherOrbitGraphs = {};
    orbitsAway++;
  }
}
console.log(sumOrbits(map));
