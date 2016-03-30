
#include <stdio.h>
#include "scanner.h"
#include "lexer.h"


void demo_scanner() {
    printf("Scanner Demo\n");

    struct VuScanner* scanner = vu_scanner_new("one two\n three");

    printf("Scanner done: %d\n", scanner->done);
    printf("Source: %s\n", scanner->source);

    while (vu_scanner_running(scanner)) {
        vu_character_t c = vu_scanner_next(scanner);
        printf("[%d:%d]: %c\n", c.line, c.column, c.val);
    }

    printf("Scanner Done\n");
    vu_scanner_free(scanner);
}


void demo_lexer() {
    printf("Lexer Demo\n");

    struct VuScanner* scanner = vu_scanner_new("4 * 3 + 2 - 1");
    vu_Lexer_t* lexer = vu_Lexer_new(scanner);

    

    vu_Lexer_free(lexer);
    vu_scanner_free(scanner);

    printf("Lexer Done\n");
}


int main() {
    demo_scanner();
    demo_lexer();
    return 0;
}