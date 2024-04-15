#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct String {
  char *s;
  int len;
};
void consume(struct String * s){
  printf("string struct: {'%s', len=%d}\n",s->s,s->len);
  free(s);
}
int main(int argc, char **argv) {
  struct String *s = malloc(sizeof(struct String));
  if (argc < 2) {
    printf("Usage: ./run <string>\n");
    return 1;
  }
  s->s = argv[1];
  s->len = strlen(s->s);
  consume(s);
  consume(s);
  return 0;
}
