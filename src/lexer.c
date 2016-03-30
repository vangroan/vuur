
#include "lexer.h"


struct VuLexer*
vu_lexer_new(struct VuScanner* scanner) {
    struct VuLexer* lexer = malloc(sizeof(struct VuLexer));
    lexer->scanner = scanner;
    lexer->done = false;
    return lexer;
}

void
vu_lexer_free(struct VuLexer* self) {
    free(self);
}

vu_Token_t*
vu_lexer_scan(struct VuLexer* self) {
    return NULL;
}