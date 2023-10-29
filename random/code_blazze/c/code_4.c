#include <stdio.h>
#include <string.h>

int value(char r) {
  if (r == 'I')
    return 1;
  if (r == 'V')
    return 5;
  if (r == 'X')
    return 10;
  if (r == 'L')
    return 50;
  if (r == 'C')
    return 100;
  if (r == 'D')
    return 500;
  if (r == 'M')
    return 1000;

  return -1;
}

int romanToDecimal(char str[]) {

  int res = 0;

  for (int i = 0; i < strlen(str); i++) {

    int s1 = value(str[i]);

    if (i + 1 < strlen(str)) {

      int s2 = value(str[i + 1]);
      printf("%d %d\n", s1, s2);

      if (s1 < s2) {

        res = res + s2 - s1;
      } else {

        res = res + s2 + s1;
        i++;
      }
    } else {
      res = res + s1;
    }
  }
  return res;
}

int main() {

  char str[18] = "MCMLXVII";
  printf("Integer form of Roman Numeral is %d", romanToDecimal(str));

  return 0;
}
