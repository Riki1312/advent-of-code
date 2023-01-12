const source = await Deno.readTextFile("day2/input.txt");
const lines = source.split("\n");

const mapPoint = {
  "A": 1,
  "B": 2,
  "C": 3,
  "X": 1,
  "Y": 2,
  "Z": 3,
};

const matrixResult = [
  [3, 0, 6],
  [6, 3, 0],
  [0, 6, 3],
];

let result = 0;
for (const line of lines) {
  const [c0, c1] = line.split(" ");
  const p0 = mapPoint[c0.trim() as keyof typeof mapPoint];
  const p1 = mapPoint[c1.trim() as keyof typeof mapPoint];

  result += p0;
  result += matrixResult[p0 - 1][p1 - 1];
}

console.log("Result:", result);
