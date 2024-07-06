main() {
  List array = [7, 12, 9, 4, 11];
  int min = findSmallestInt(array);
  print(min);
}

int findSmallestInt(List val) {
  int minVal = val[0];
  for (int i in val) {
    if (i < minVal) {
      minVal =  i;
    }
  }
  return minVal;
}
