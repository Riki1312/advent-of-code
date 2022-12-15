import 'dart:io';

void main() {
  final input = File("test.txt").readAsStringSync().trim();

  final result = input
      .split("\n\r")
      .map((s) => s.split("\n"))
      .map((l) => l.where((e) => e.trim().isNotEmpty).map((e) => int.parse(e)))
      .map((l) => l.reduce((a, b) => a + b))
      .toList()
    ..sort();

  print(result.last);
}
