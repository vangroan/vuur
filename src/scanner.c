

#include "scanner.h"

vu_scanner_t* vu_scanner_new(char* source) {
    vu_scanner_t* self = (vu_scanner_t*)malloc(sizeof(vu_scanner_t));
    self->source = source;
    self->position = 0;
    self->done = vu_False;
    return self;
}

void vu_scanner_free(vu_scanner_t* self) {
    free(self);
}

vu_character_t vu_scanner_next(vu_scanner_t* self) {
    vu_character_t c;
    c.val = self->source[self->position];
    self->position++;
    self->done = vu_True;
    return c;
}

vu_Bool vu_scanner_running(vu_scanner_t* self) {
    return !self->done;
}