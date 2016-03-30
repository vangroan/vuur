
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

static inline bool
isWhitespace(const char c) {
    return c == ' ' || c == '\t' || c == '\n' || c == '\r';
}

static inline bool
isEOF(struct VuCharacter* chr) {
    return chr->kind == vu_eof;
}

static inline void
lexer_next_character(struct VuLexer* self) {
    self->character = vu_scanner_next(self->scanner);
}

static inline char
lexer_char(struct VuLexer* self) {
    return self->character.val;
}

static inline bool
lexer_checkEOF(struct VuLexer* self) {
    if (isEOF(&self->character)) {
        self->done = true;
        return true;
    }
    return false;
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

    lexer->done = false;

    return lexer;
}


void
vu_lexer_free(struct VuLexer* self) {
    free(self->current);
    free(self);
}


struct VuToken*
vu_lexer_next(struct VuLexer* self) {
    lexer_next_character(self);    

    // Ignore Whitespace
    while(isWhitespace(lexer_char(self)) && vu_scanner_running(self->scanner)) {
        lexer_next_character(self);
    }

    return self->current;
}


bool vu_lexer_running(struct VuLexer* self) {
    return !self->done;
}