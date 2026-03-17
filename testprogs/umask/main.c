

#include <stdio.h>
#include <stdlib.h>
int main(int argc, char* argv[]) {
	FILE* fptr;

	printf("aaaa\n");
	fptr = fopen("test.txt", "w");

	fprintf(fptr, "test message");

	fclose(fptr);
	return EXIT_SUCCESS;
}
