extension StringExtension on String {
  List<String> splitOnEmptyLine() {
    return this.split(RegExp(r"(\n\n)|(\n\r)"));
  }
}
