import 'dart:collection';

import 'package:quiver/iterables.dart';

/// A range of numbers.
class Range<T extends num> {
  /// The start of the range.
  final T start;

  /// The end of the range.
  final T end;

  /// Creates a range from [start] to [end].
  Range(this.start, this.end);

  /// Creates a range from a [list] of two elements.
  Range.fromList(List<T> list) : this(list[0], list[1]);

  /// Checks if the range overlaps with [other] range.
  bool overlaps(Range<T> other) {
    return start <= other.end && end >= other.start;
  }

  /// Checks if the range contains [other] range.
  bool contains(Range<T> other) {
    return start <= other.start && end >= other.end;
  }
}

class Stack<T> {
  final _queue = Queue<T>();

  bool get isEmpty => _queue.isEmpty;
  bool get isNotEmpty => _queue.isNotEmpty;
  int get length => this._queue.length;

  void clear() {
    _queue.clear();
  }

  void push(T element) {
    _queue.addLast(element);
  }

  T? pop() {
    return this.isEmpty ? null : _queue.removeLast();
  }

  T? peek() {
    return this.isEmpty ? null : _queue.last;
  }

  @override
  String toString() {
    return _queue.toString();
  }
}

/// Extension methods for [String].
extension StringExtension on String {
  /// Splits the string on empty lines.
  List<String> splitOnEmptyLine() {
    return this.split(RegExp(r"(\n\n)|(\n\r)"));
  }
}

/// Extension methods for [Iterable].
extension IterableExtension<T> on Iterable<T> {
  /// Splits the iterable into lists of the given [length].
  Iterable<List<T>> split(int length) => partition(this, length);
}
