#include <stdio.h>
#include <string.h>

#define MAX_BUF 20

int main(int argc, char **argv) {
  const char secret[] = "letmein";
  char buf[MAX_BUF];
  if (argc < 2){
    printf("Usage: ./run <guess>\n");
    return 1;
  }
  if (argc > 1) {
    strcpy(buf, argv[1]);
  }
  if (0 == strncmp(buf,secret,sizeof(secret))) {
    printf("You guessed correctly!\n");
    printf("The secret was: %s \n",secret);
    return 0;
  }
  printf("'%s' was not the secret.\nTry again later!\n",buf);
  return 0;
}
