

#include "scanner.h"

// -------
// Private
// -------

static struct VuCharacter
create_null_character(const int pos, const int line, const int col) {
    return (struct VuCharacter){
        '\0', // val
        pos,
        line,
        col,
        vu_eof
    };
}


// Mark scanner as done
static void
scanner_finish(struct VuScanner* self) {
    self->done = vu_True;
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
    self->done = vu_False;
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

    // TODO: Character kinds
    c.kind = vu_char;

    return c;
}


vu_Bool 
vu_scanner_running(const struct VuScanner* self) {
    return !self->done;
}