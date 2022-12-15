import 'dart:io';

import '../utils.dart';

var scoreMap = [
  [1 + 3, 1 + 0, 1 + 6],
  [2 + 6, 2 + 3, 2 + 0],
  [3 + 0, 3 + 6, 3 + 3]
];

var letterMap = {
  "A": 0,
  "B": 1,
  "C": 2,
  "X": 0,
  "Y": 1,
  "Z": 2,
};

void main() {
  final input = File("input.txt").readAsStringSync().trim();

  final result = input
      .split("\n")
      .map((s) => s.split(" ").map((s) => s.trim()).toList())
      .map((l) {
    int i = letterMap[l[0]]!;
    int j = letterMap[l[1]]!;

    return scoreMap[j][i];
  });

  print(result.sum);
}
