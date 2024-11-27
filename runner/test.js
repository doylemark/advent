const fs = require("fs");

const contents = fs.readFileSync("./input.txt", { encoding: "utf-8" });

const lines = contents.split("\n");

let prev = Number(lines[0]);
let n = 0;
for (let i = 0; i < lines.length; i++) {
  if (Number(lines[i]) > prev) {
    n++;
  }

  prev = Number(lines[i]);
}

console.log({ n });
