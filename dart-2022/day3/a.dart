import 'dart:io';

import '../utils.dart';

void main() {
  final input = File("input.txt").readAsStringSync().trim();

  final result = input
      .split("\n")
      .map((s) => s.split(""))
      .map((l) => [l.sublist(0, l.length ~/ 2), l.sublist(l.length ~/ 2)])
      .map((l) => l[0].where((e) => l[1].contains(e)).first)
      .map((e) {
        if (e.toUpperCase() == e) {
          return e.codeUnits.first - "A".codeUnits.first + 27;
        } else {
          return e.codeUnits.first - "a".codeUnits.first + 1;
        }
      })
      .toList()
      .sum;

  print(result);
}
