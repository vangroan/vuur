
#ifndef VUUR_SCANNER
#define VUUR_SCANNER

#include <stdlib.h>
#include "types.h"

typedef struct {
    char* source;
    int position;
    vu_Bool done;
} vu_scanner_t;

typedef struct {
    char val;
} vu_character_t;

vu_scanner_t* vu_scanner_new(char* source) ;
void vu_scanner_free(vu_scanner_t* self);
vu_character_t vu_scanner_next(vu_scanner_t* self);
vu_Bool vu_scanner_running(vu_scanner_t* self);

#endif