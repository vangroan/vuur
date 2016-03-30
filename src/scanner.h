
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


typedef struct {
    char* source;
    int sourceLength;
    int position;
    int line;
    int column;
    vu_Bool done;
} vuScanner;


typedef struct {
    char val;
    int position;
    int line;
    int column;
    enum vu_character_kind kind;
} vu_character_t;


vuScanner* vuScanner_new(const char* source) ;
void vu_scanner_free(vuScanner* self);
vu_character_t vu_scanner_next(vuScanner* self);
vu_Bool vu_scanner_running(vuScanner* self);


#endif