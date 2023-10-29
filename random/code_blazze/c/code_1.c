#include <stdio.h>
int main() {
  int i, j, k, flag = 0, n;

  int cloud[10000];
  
  printf("Enter array size: ");
  scanf("%d", &n);
  int a[n];
  for (int i=0; i<10000; i++) {
    cloud[i] = 0;
  }

  int count = 0;
  for (i = 0; i < n; i++) {
    int c;
    scanf("%d", &c);
    if (cloud[c] == 0) {
      cloud[c] = 1;
      a[count] = c;
      count ++;
    }
  }

  for (i = 0; i < count; i++) {
    printf("%d ", a[i]);
  }

  return 0;
}
