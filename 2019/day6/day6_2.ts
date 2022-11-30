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

function pathToCOM(target: string, graph: Object): Array<string> {
  if (graph === true) return [];
  for (const key in graph) {
    if (key === target) {
      console.log(target);
      return [target];
    }
    const tempPath = pathToCOM(target, graph[key].directlyOrbitedBy);
    if (tempPath.length !== 0) {
      return [...tempPath, key];
    }
  }
  return [];
}

let pathYou = [];
let pathSanta = [];
while (pathYou[pathYou.length - 1] !== "COM") {
  pathYou = pathToCOM("YOU", map);
}
while (pathSanta[pathSanta.length - 1] !== "COM") {
  pathSanta = pathToCOM("SAN", map);
}

let planetFound = false;
for (const santaPlanet of pathSanta) {
  for (const youPlanet of pathYou) {
    if (youPlanet === santaPlanet) {
      planetFound = true;
      console.log(youPlanet)
      console.log(
        pathSanta.indexOf(youPlanet) + pathYou.indexOf(youPlanet) - 2
      );
      break;
    }
  }
  if (planetFound) break;
}
