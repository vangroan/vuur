
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
} vu_scanner_t;


typedef struct {
    char val;
    int position;
    int line;
    int column;
    enum vu_character_kind kind;
} vu_character_t;


vu_scanner_t* vu_scanner_new(char* source) ;
void vu_scanner_free(vu_scanner_t* self);
vu_character_t vu_scanner_next(vu_scanner_t* self);
vu_Bool vu_scanner_running(vu_scanner_t* self);


#endif