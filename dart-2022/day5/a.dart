import 'dart:io';

import '../utils.dart';

void main() {
  final input = File("input.txt").readAsStringSync();

  final count =
      input.splitOnEmptyLine().first.split("\n").last.split("   ").length;

  print(count);

  final values = input
      .splitOnEmptyLine()
      .first
      .split("\n")
      .reversed
      .skip(1)
      .map((s) => s.replaceAll("    ", "0"))
      .map((s) => RegExp(r"[A-Z]|0").allMatches(s))
      .map((m) => m.take(count).map((m) => m[0]!))
      .map((l) => l.toList());

  //print(values);

  final stacks = <Stack<String>>[];

  for (List<String> list in values) {
    for (int i = 0; i < list.length; i++) {
      if (list[i] == "0") {
        continue;
      }
      if (stacks.length <= i) {
        stacks.add(Stack<String>());
      }

      stacks[i].push(list[i]);
    }
  }

  //print(stacks);

  final moves = input
      .splitOnEmptyLine()
      .last
      .split("\n")
      .skip(1)
      .map((s) => RegExp(r"\d+").allMatches(s))
      .map((m) => m.take(3).map((m) => m[0]!))
      .map((l) => l.map(int.parse).toList());

  //print(moves);

  for (List<int> move in moves) {
    final count = move[0];
    final from = move[1] - 1;
    final to = move[2] - 1;

    for (int i = 0; i < count; i++) {
      stacks[to].push(stacks[from].pop()!);
    }
  }

  //print(stacks);

  var result = "";
  for (Stack stack in stacks) {
    result += stack.peek();
  }

  print(result);
}
