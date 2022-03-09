#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define HEAP_SIZE 65536
static char heap[HEAP_SIZE];
static char * brk = NULL;

void * naive_sbrk(intptr_t inc) {
    if (inc < 0) return (void *)-1;
    if (brk == NULL) brk = heap;

    void * block = (void *)brk;
    brk += inc;

    return block;
}

void panic(char * msg) {
    puts(msg);
    exit(1);
}
