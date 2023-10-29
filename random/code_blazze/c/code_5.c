#include <stdio.h>
int main() {
  int i, j, n = 6;

  for (i = 0; i < n; i++) {
    for (j = 0; j < (2 * n); j++) {
      if (i >= j)
        printf("*");
      else
        printf(" ");
      if (i+j >= (2 * n - 1))
        printf("*");
      else
        printf(" ");
    }
    printf("\n");
  }

  for (i = 0; i < n; i++) {
    for (j = 0; j < (2 * n); j++) {
      if (i <= n - 1)
        printf(" ");
      else
        printf("+");
      if ((i + n) <= j)
        printf("*");
      else
        printf(" ");
    }
    printf("\n");
  }
  return 0;
}
