

#include "scanner.h"

// -------
// Private
// -------

static inline struct VuCharacter
create_null_character(const int pos, const int line, const int col) {
    return (struct VuCharacter){
        '\0', // val
        pos, // position
        line, // line
        col, // column
        NULL, // content
        vu_eof // done
    };
}


static inline enum vu_character_kind
choose_character_kind(const char chr) {
    switch (chr) {
        case ' ':
        case '\n':
        case '\r':
        case '\t':
            return vu_whitespace;
        break;
        default:
            return vu_char;
        break;
    }
}


// Mark scanner as done
static inline void
scanner_finish(struct VuScanner* self) {
    self->done = true;
}


static inline char*
getCharacterPointer(const struct VuScanner* self, const int position) {
    return self->source + position;
}


// ------
// Public
// ------

struct VuScanner*
vu_scanner_new(const char* source) {
    struct VuScanner* self = malloc(sizeof(struct VuScanner));
    self->source = (char*)source;
    self->sourceLength = strlen(source);
    self->position = -1;
    self->line = 0;
    self->column = -1;
    self->done = false;
    return self;
}


void
vu_scanner_free(struct VuScanner* self) {
    free(self);
}


struct VuCharacter
vu_scanner_next(struct VuScanner* self) {

    self->position++;

    if (self->position >= self->sourceLength) {
        scanner_finish(self);
        // Incrementing column to avoid having same column as previous character
        return create_null_character(self->position, self->line, ++self->column);
    }

    struct VuCharacter c;
    c.val = self->source[self->position];

    if (c.val == '\n') {
        self->column = 0;
        self->line++;
    } else {
        self->column++;
    }

    c.position = self->position;
    c.line = self->line;
    c.column = self->column;
    c.content = getCharacterPointer(self, c.position);
    c.kind = choose_character_kind(c.val);

    return c;
}


bool
vu_scanner_running(const struct VuScanner* self) {
    return !self->done;
}

void vu_character_init(struct VuCharacter* self) {
    self->val = '\0';
    self->position = 0;
    self->line = 0;
    self->column = 0;
    self->content = NULL;
    self->kind = vu_none;
}
