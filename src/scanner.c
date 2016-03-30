

#include "scanner.h"

struct VuScanner* 
vu_scanner_new(const char* source) {
    struct VuScanner* self = malloc(sizeof(struct VuScanner));
    self->source = (char*)source;
    self->sourceLength = strlen(source);
    self->position = -1;
    self->line = 0;
    self->column = -1;
    self->done = vu_False;
    return self;
}


void 
vu_scanner_free(struct VuScanner* self) {
    free(self);
}


vu_character_t 
vu_scanner_next(struct VuScanner* self) {

    if (self->position >= self->sourceLength) {
        self->done = vu_True;
        vu_character_t null_c;
        null_c.val = '\0';
        null_c.kind = vu_eof;
        return null_c;
    }

    self->position++;

    vu_character_t c;
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

    // TODO: Character kinds
    c.kind = vu_char;

    return c;
}


vu_Bool 
vu_scanner_running(const struct VuScanner* self) {
    return !self->done;
}