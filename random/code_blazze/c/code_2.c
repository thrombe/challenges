#include <stdio.h>
#include <string.h>
int main() {
  char str[100];
  char out[100];
  int n, i, j, c = 0;
  printf("Enter a string\n");
  gets(str);
  n = strlen(str);
  // printf("%s", str);

  int count = 0;

  int space = 0;
  for (int i=0; i<n; i++) {
    char c = str[i];
    if (c == ' ') {
      if (space == 0) {
        out[count] = c;
        count++;
      }
      space = 1;
    } else {
      space = 0;
      out[count] = c;
      count++;
    }
  }
  out[count+1] = '\0';

  
  // for (i = 0; i < n; i++) {
  //   if (str[i] == ' ' && str[i + 1] == ' ') {
  //     i++;
  //     c++;
  //     for (j = (i + 1); j < n - c; j++) {
  //       str[j] = str[j + 1];
  //     }
  //     str[j] = '\0';
  //   }
  // }
  printf("The Modified string is - ");
  puts(out);
  return 0;
}
