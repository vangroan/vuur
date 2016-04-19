
#include "lexer.h"

// -------
// Private
// -------


static void
dumpToken(struct VuToken* token) {
    char buffer[128];
    const size_t max = 127;
    size_t length = token->length > max ? max : token->length;
    memcpy(&buffer, token->content, length);
    buffer[length] = '\0';
    printf("<Token %d:%d:%d '%s' length:%d >\n",
        token->position,
		token->line,
		token->column,
        buffer,
		token->length
    );
}

static void
dumpCharacter(struct VuCharacter* chr) {
    char* kind = NULL;

    switch(chr->kind) {
        case vu_none: kind = "none"; break;
        case vu_char: kind = "char"; break;
        case vu_whitespace: kind = "whitespace"; break;
        case vu_eof: kind = "eof"; break;
        default: kind = "unknown"; break;
    }

    printf("<Character %c %d %s >\n",
        chr->content == NULL ? ' ' : chr->content[0],
        chr->position,
        kind
    );
}

static inline void
init_empty_token(struct VuToken* token) {
    token->kind = TOKEN_NOTOKEN;
    token->position = 0;
    token->line = 0;
    token->column = 0;
    token->content = NULL;
    token->length = 0;
}

static inline bool
isNone(const struct VuCharacter* chr) {
    return chr->kind == vu_none;
}

static inline bool
isWhitespace(const char c) {
    return c == ' ' || c == '\t' || c == '\r';
}

static inline bool
isNewline(const char c) {
    return c == '\n';
}

static inline bool
isNumber(const char c) {
    return '0' <= c && c <= '9';
}

static inline bool
isUnderscore(const char c) {
    return c == '_';
}

static inline bool
isLetter(const char c) {
    return ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z');
}

static inline bool
isOpenBracket(const char c) {
    return c == '(';
}

static inline bool
isCloseBracket(const char c) {
    return c == ')';
}

static inline bool
isEOF(struct VuCharacter* chr) {
    return chr->kind == vu_eof;
}

static inline void
lexer_next_character(struct VuLexer* self) {
    self->character = vu_scanner_next(self->scanner);
}

static inline void
lexer_consume(struct VuLexer* self) {
    self->current->length++;
    lexer_next_character(self);
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

static void
lexer_readUntilWhitespace(struct VuLexer* self) {
    printf("Reading until whitespace\n");
    while ((isLetter(lexer_char(self)) || isNumber(lexer_char(self))
            || isUnderscore(lexer_char(self))) && !lexer_checkEOF(self)) {
        lexer_consume(self);
    }
}


static void
lexer_makeToken(struct VuLexer* self) {
    self->current->kind = TOKEN_NOTOKEN;
    self->current->position = self->character.position;
    self->current->line = self->character.line;
    self->current->column = self->character.column;
    self->current->content = self->character.content;
    self->current->length = 0;
}


static void
lexer_keyword(struct VuLexer* self) {
    // TODO
}


// ------
// Public
// ------


struct VuLexer*
vu_lexer_new(struct VuScanner* scanner) {
    struct VuLexer* lexer = malloc(sizeof(struct VuLexer));

    lexer->scanner = scanner;

    vu_character_init(&lexer->character);

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
    dumpCharacter(&self->character);
    // Ignore leading whitespace
    while ((isWhitespace(lexer_char(self)) || isNone(&self->character))
        && vu_scanner_running(self->scanner)) {
        lexer_next_character(self);
    }

    if (isLetter(lexer_char(self))) {
        lexer_makeToken(self);
        lexer_readUntilWhitespace(self);
        lexer_keyword(self);
    } else if (isOpenBracket(lexer_char(self))) {
        lexer_makeToken(self);
		lexer_consume(self);
        lexer_next_character(self);
    } else if (isCloseBracket(lexer_char(self))) {
        lexer_makeToken(self);
		lexer_consume(self);
        lexer_next_character(self);
    } else if (isNewline(lexer_char(self))) {
        lexer_makeToken(self);
		lexer_consume(self);
        lexer_next_character(self);
    }

    dumpToken(self->current);

    return self->current;
}


bool vu_lexer_running(struct VuLexer* self) {
    return !self->done;
}
