import 'dart:io';

import 'package:tuple/tuple.dart';

import '../utils.dart';

void main() {
  final input = File("test.txt").readAsStringSync().trim();

  final result = input
      .split("\n")
      .map((l) => l.split(","))
      .map((l) {
        final range1 = l[0].split("-").map(int.parse).toList();
        final range2 = l[1].split("-").map(int.parse).toList();

        return Tuple2(Range.fromList(range1), Range.fromList(range2));
      })
      .where((e) => e.item1.contains(e.item2) || e.item2.contains(e.item1))
      .length;

  print(result);
}
