const source = await Deno.readTextFile("day2/input.txt");
const data = source.split("\n").map((x) => {
  const line = x.split(" ");
  return [line[0], line[1]];
});

console.log(data);

let position = 0;
let depth = 0;
let aim = 0;

for (const [action, v] of data) {
  const value = parseInt(v, 10);

  if (action === "forward") {
    position += value;
    depth += aim * value;
  } else if (action === "down") {
    aim += value;
  } else if (action === "up") {
    aim -= value;
  }
}

console.log(position * depth);
