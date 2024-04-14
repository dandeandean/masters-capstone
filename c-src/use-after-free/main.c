#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct String {
  char *s;
  int len;
};

int main(int argc, char **argv) {
  struct String *s = malloc(sizeof(struct String));
  if (argc < 2) {
    printf("Usage: ./run <string>\n");
    return 1;
  }
  s->s = argv[1];
  s->len = strlen(s->s);
  printf("String(%s,%d)\n", s->s,s->len);
  char option;    
  while (option != 'q') {
    printf("What to do next?\n");
    printf("(f)ree\n(p)rint\n(q)uit\n>");
    scanf("%c",&option);
    switch (option)
    {
    case 'f':
      system("clear");
      printf("\nFreeing...\n");
      free(s);
      break;
    case 'p':
      system("clear");
      printf("\nString(%s,%d)\n\n", s->s,s->len);
      break;
    default:
      break;
    }
  }
  return 0;
}
