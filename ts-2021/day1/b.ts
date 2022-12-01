const source = await Deno.readTextFile("day1/input.txt");
const data = source.split("\n").map((x) => parseInt(x, 10));

let increments = 0;

console.log(data);

for (let i = 0; i < data.length - 3; i++) {
  if (data[i] < data[i + 3]) increments++;
}

console.log(increments);
