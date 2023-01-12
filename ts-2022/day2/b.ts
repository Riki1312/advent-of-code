const source = await Deno.readTextFile("day1/input.txt");
const lines = source.split("\n");

const data: number[] = [0];
let dataIndex = 0;

for (const line of lines) {
  const value = parseInt(line);

  if (isNaN(value)) {
    dataIndex++;
    data.push(0);
  } else {
    data[dataIndex] += value;
  }
}

console.log(data);

const max1 = Math.max(...data);
const max2 = Math.max(...data.filter((x) => x !== max1));
const max3 = Math.max(...data.filter((x) => x !== max1 && x !== max2));

console.log(max1, max2, max3);
console.log(max1 + max2 + max3);
