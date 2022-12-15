import 'dart:io';

import 'package:basics/basics.dart';

var scoreMap = [
  [1 + 3, 1 + 0, 1 + 6],
  [2 + 6, 2 + 3, 2 + 0],
  [3 + 0, 3 + 6, 3 + 3]
];

var letterMap = {
  "A": 0,
  "B": 1,
  "C": 2,
};

var actionMap = {
  "A": {
    "X": "C",
    "Y": "A",
    "Z": "B",
  },
  "B": {
    "X": "A",
    "Y": "B",
    "Z": "C",
  },
  "C": {
    "X": "B",
    "Y": "C",
    "Z": "A",
  },
};

void main() {
  final input = File("input.txt").readAsStringSync().trim();

  final result = input
      .split("\n")
      .map((s) => s.split(" ").map((s) => s.trim()).toList())
      .map((l) {
    int i = letterMap[l[0]]!;
    String a = actionMap[l[0]]![l[1]]!;
    int j = letterMap[a]!;

    return scoreMap[j][i];
  });

  print(result.sum());
}
