

#ifndef VUUR_LEXER
#define VUUR_LEXER

#include <stdbool.h>

#include "scanner.h"
#include "token.h"


struct VuLexer {
    struct VuScanner* scanner;
    bool done;
};


struct VuLexer* vu_lexer_new(struct VuScanner* scanner);
void vu_lexer_free(struct VuLexer* self);
vu_Token_t* vu_lexer_scan(struct VuLexer* self);

#endif