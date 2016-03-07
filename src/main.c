
#include <stdio.h>
#include "scanner.h"

void demo_scanner() {
    printf("Scanner Demo\n");

    vu_scanner_t* scanner = vu_scanner_new("one two three");

    printf("Scanner done: %d\n", scanner->done);
    printf("Source: %s\n", scanner->source);

    while (vu_scanner_running(scanner)) {
        vu_character_t c = vu_scanner_next(scanner);
        printf("Char: %c\n", c.val);
    }

    printf("Scanner Done\n");
    vu_scanner_free(scanner);
}

int main() {
    demo_scanner();
    return 0;
}