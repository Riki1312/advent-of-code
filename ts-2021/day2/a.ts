const source = await Deno.readTextFile("day2/input.txt");
const data = source.split("\n").map((x) => {
  const line = x.split(" ");
  return [line[0], line[1]];
});

console.log(data);

let position = 0;
let depth = 0;

for (const [action, v] of data) {
  const value = parseInt(v, 10);

  if (action === "forward") {
    position += value;
  } else if (action === "down") {
    depth += value;
  } else if (action === "up") {
    depth -= value;
  }
}

console.log(position * depth);
