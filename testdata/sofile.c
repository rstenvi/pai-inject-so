#define _GNU_SOURCE

#include <stdio.h>
#include <dlfcn.h>
#include <unistd.h>

int (*orig_puts)(const char *str1);

void __attribute__ ((constructor)) setup(void) {
	printf("constructor was called\n");
}

int puts(const char *str) {
  if(!orig_puts) orig_puts = dlsym(RTLD_NEXT, "puts");
  write(1, "prog wrote: ", 12);
  orig_puts(str);
  return 0;
}
