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

console.log(max1);
