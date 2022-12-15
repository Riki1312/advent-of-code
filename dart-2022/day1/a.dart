import 'dart:io';

import '../utils.dart';

void main() {
  final input = File("input.txt").readAsStringSync().trim();

  final result = input
      .splitOnEmptyLine()
      .map((s) => s.split("\n"))
      .map((l) => l.where((e) => e.trim().isNotEmpty).map((e) => int.parse(e)))
      .map((l) => l.reduce((a, b) => a + b))
      .toList()
    ..sort();

  print(result.last);
}
