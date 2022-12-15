import 'dart:io';

import 'package:basics/basics.dart';
import 'package:characters/characters.dart';

import '../utils.dart';

void main() {
  final input = File("input.txt").readAsStringSync().trim();

  final result = input
      .split("\n")
      .split(3)
      .map((g) => g[0]
          .characters
          .where((c) => g[1].contains(c) && g[2].contains(c))
          .first)
      .map((e) {
    if (e.toUpperCase() == e) {
      return e.codeUnits.first - "A".codeUnits.first + 27;
    } else {
      return e.codeUnits.first - "a".codeUnits.first + 1;
    }
  }).sum();

  print(result);
}
