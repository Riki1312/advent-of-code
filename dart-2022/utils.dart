extension StringExtension on String {
  /// Splits the string on empty lines.
  List<String> splitOnEmptyLine() {
    return this.split(RegExp(r"(\n\n)|(\n\r)"));
  }
}

extension IterableExtension<T> on Iterable<T> {
  /// Returns the sum of all numbers in the iterable.
  num get sum {
    return this.fold(0, (accumulator, value) {
      if (value is num) {
        return accumulator + value;
      } else {
        return accumulator;
      }
    });
  }

  /// Splits the iterable into chunks of a given length.
  Iterable<List<T>> split(int length) sync* {
    var i = 0;
    while (i < this.length) {
      yield this.skip(i).take(length).toList();
      i += length;
    }
  }
}
