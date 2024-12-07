const fs = require("fs");

const contents = fs.readFileSync("./input.txt", { encoding: "utf-8" });

const x = contents.match(/SAMX/g).length;

console.log(x);

// const lines = contents.split("\n");
//
//
