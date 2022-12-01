const source = await Deno.readTextFile("day3/input.txt");

const count0 = Array.from({ length: 12 }, () => 0);
const count1 = Array.from({ length: 12 }, () => 0);

source.split("\n").forEach((line) => {
  const bits = line.split("");

  for (let i = 0; i < bits.length; i++) {
    if (bits[i] == "0") {
      count0[i]++;
    } else if (bits[i] == "1") {
      count1[i]++;
    }
  }
});

console.log(count0);
console.log(count1);

let gamma = "";
let epsilon = "";

for (let i = 0; i < count0.length; i++) {
  if (count0[i] > count1[i]) {
    gamma += "0";
    epsilon += "1";
  } else {
    gamma += "1";
    epsilon += "0";
  }
}

console.log(gamma, epsilon);

const result = parseInt(gamma, 2) * parseInt(epsilon, 2);

console.log(result);
