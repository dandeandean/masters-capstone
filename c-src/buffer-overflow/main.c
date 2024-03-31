#include <stdio.h>
// #include <stdlib.h>
#include <string.h>

#define MAX_BUF 5

int main(int argc, char **argv) {
  char buf[MAX_BUF];
  if (argc > 1) {
    strcpy(buf, argv[1]);
  }
  printf("\n");
}
