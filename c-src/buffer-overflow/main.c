// #include <CoreGraphics/CoreGraphics.h>
#include <stdio.h>
#include <stdlib.h>

#define MAX_BUF 5

int main(int argc, char **argv) {
  int buf[MAX_BUF];
  for (int i = 1; i < argc; i++) {
    buf[i - 1] = atoi(argv[i]);
  }
  printf("\n");
}
