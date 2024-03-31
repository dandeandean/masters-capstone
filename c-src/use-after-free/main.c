#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct String {
  char *s;
  int len;
};

int main(int argc, char **argv) {
  struct String *s = malloc(sizeof(struct String));
  s->s = "hello";
  s->len = strlen(s->s);
  printf("%s\n", s->s);
  printf("Freeing s\n");
  free(s);
  printf("%s\n", s->s);
  return 0;
}
