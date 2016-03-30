
#ifndef VUUR_SCANNER
#define VUUR_SCANNER

#include <stdlib.h>
#include <string.h>
#include "types.h"


enum vu_character_kind {
    vu_char,
    vu_space,
    vu_eof
};


struct VuScanner {
    char* source;
    int sourceLength;
    int position;
    int line;
    int column;
    vu_Bool done;
};


struct VuCharacter {
    char val;
    int position;
    int line;
    int column;
    enum vu_character_kind kind;
};


struct VuScanner* vu_scanner_new(const char* source) ;
void vu_scanner_free(struct VuScanner* self);
struct VuCharacter vu_scanner_next(struct VuScanner* self);
vu_Bool vu_scanner_running(const struct VuScanner* self);


#endif