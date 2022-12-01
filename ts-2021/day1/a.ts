const source = await Deno.readTextFile("day1/input.txt");
const data = source.split("\n").map((x) => parseInt(x, 10));

let lastValue = Infinity;
let increments = 0;

console.log(data);

for (const value of data) {
  if (value > lastValue) {
    increments++;
  }

  lastValue = value;
}

console.log(increments);
