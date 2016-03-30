
#include <stdbool.h>
#include <stdio.h>

#include "scanner.h"
#include "lexer.h"


void assert(bool condition, const char* message) {
    if (!condition) {
        printf("Assertion failed: %s", message);
        exit(1);
    }
}


const char* load_source(const char* filepath) {
    FILE* fp = fopen(filepath, "r");
    long filesize = 0;
    char* buffer = NULL;

    fseek(fp, 0, SEEK_END);
    filesize = ftell(fp);
    rewind(fp);

    buffer = malloc(filesize * sizeof(char));
    fread(buffer, sizeof(char), filesize, fp);

    fclose(fp);

    return buffer;
}


void demo_scanner() {
    printf("Scanner Demo\n");

    struct VuScanner* scanner = vu_scanner_new("one two\n three");

    printf("Scanner done: %d\n", scanner->done);
    printf("Source: %s\n", scanner->source);

    while (vu_scanner_running(scanner)) {
        struct VuCharacter c = vu_scanner_next(scanner);
        printf("[%d:%d]: %c\n", c.line, c.column, c.val);
    }

    printf("Scanner Done\n");
    vu_scanner_free(scanner);
}


void demo_lexer() {
    printf("Lexer Demo\n");
    const char* source = load_source("samples/procedure.vu");

    struct VuScanner* scanner = vu_scanner_new(source);
    struct VuLexer* lexer = vu_lexer_new(scanner);

    vu_lexer_next(lexer);

    vu_lexer_free(lexer);
    vu_scanner_free(scanner);
    free((char*)source);

    printf("Lexer Done\n");
}


int main() {
    demo_scanner();
    demo_lexer();
    return 0;
}