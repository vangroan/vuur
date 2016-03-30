

#include "scanner.h"

vuScanner* vuScanner_new(char* source) {
    vuScanner* self = malloc(sizeof(vuScanner));
    self->source = source;
    self->sourceLength = strlen(source);
    self->position = -1;
    self->line = 0;
    self->column = -1;
    self->done = vu_False;
    return self;
}

void vu_scanner_free(vuScanner* self) {
    free(self);
}

vu_character_t vu_scanner_next(vuScanner* self) {

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

vu_Bool vu_scanner_running(vuScanner* self) {
    return !self->done;
}