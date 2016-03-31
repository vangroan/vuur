

#ifndef VUUR_LEXER
#define VUUR_LEXER

#include <stdbool.h>
#include <stdio.h>

#include "scanner.h"


enum VuTokenKind {
    TOKEN_NOTOKEN,
    TOKEN_PLUS,
    TOKEN_MINUS,

    TOKEN_NUMLITERAL,

    TOKEN_PROCEDURE,

    TOKEN_LINEBREAK
};


struct VuToken {
    enum VuTokenKind kind;

    // Position in source
    int position;

    int line;

    int column;

    // Pointer into source where token content starts
    char* content;

    // The length of the token's content
    int length;
};


struct VuLexer {
    struct VuScanner* scanner;

    struct VuCharacter character;

    struct VuToken* current;

    bool done;
};


struct VuLexer* vu_lexer_new(struct VuScanner* scanner);
void vu_lexer_free(struct VuLexer* self);
struct VuToken* vu_lexer_next(struct VuLexer* self);
bool vu_lexer_running(struct VuLexer* self);

#endif
