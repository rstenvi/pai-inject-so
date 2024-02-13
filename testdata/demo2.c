#define _GNU_SOURCE
#include <stdio.h>
#include <unistd.h>

int main() {
	int i;
	for(i = 0; i < 100; i++) {
		puts("Hello World!");
		sleep(1);
	}
	return 0;
}
