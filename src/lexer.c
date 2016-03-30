
#include "lexer.h"

// -------
// Private
// -------


static inline void
init_empty_token(struct VuToken* token) {
    token->kind = TOKEN_NOTOKEN;
    token->content = NULL;
    token->length = 0;
}


// ------
// Public
// ------


struct VuLexer*
vu_lexer_new(struct VuScanner* scanner) {
    struct VuLexer* lexer = malloc(sizeof(struct VuLexer));
    
    lexer->scanner = scanner;

    lexer->current = malloc(sizeof(struct VuToken));
    init_empty_token(lexer->current);

    return lexer;
}

void
vu_lexer_free(struct VuLexer* self) {
    free(self);
}

struct VuToken*
vu_lexer_scan(struct VuLexer* self) {
    return NULL;
}