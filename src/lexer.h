

#ifndef VUUR_LEXER
#define VUUR_LEXER

#include <stdbool.h>

#include "scanner.h"


enum VuTokenKind {
    TOKEN_NOTOKEN,
    TOKEN_PLUS,
    TOKEN_MINUS,

    TOKEN_NUMLITERAL
};


struct VuToken {
    enum VuTokenKind kind;

    // Pointer into source where token content starts
    char* content;

    // The length of the token's content
    int length;
};


struct VuLexer {
    struct VuScanner* scanner;

    struct VuToken* current;

    bool done;
};


struct VuLexer* vu_lexer_new(struct VuScanner* scanner);
void vu_lexer_free(struct VuLexer* self);
struct VuToken* vu_lexer_scan(struct VuLexer* self);

#endif