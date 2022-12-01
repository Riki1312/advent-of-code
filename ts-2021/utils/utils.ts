export async function fileToLines(path: string): Promise<string[]> {
  return (await Deno.readTextFile(path)).split("/n");
}

export async function fileToWords(path: string): Promise<string[]> {
  return (await fileToLines(path)).flatMap((line) => line.split(" "));
}
