
#ifndef VUUR_SCANNER
#define VUUR_SCANNER

#include <stdbool.h>
#include <stdlib.h>
#include <string.h>


enum vu_character_kind {
    vu_char,
    vu_whitespace,
    vu_eof
};


struct VuScanner {
    char* source;
    int sourceLength;
    int position;
    int line;
    int column;
    bool done;
};


struct VuCharacter {
    char val;
    int position;
    int line;
    int column;
    enum vu_character_kind kind;
};


struct VuScanner* vu_scanner_new(const char* source);
void vu_scanner_free(struct VuScanner* self);
struct VuCharacter vu_scanner_next(struct VuScanner* self);
bool vu_scanner_running(const struct VuScanner* self);


#endif