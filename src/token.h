
#ifndef VUUR_TOKEN
#define VUUR_TOKEN


#include <stdlib.h>


enum vu_Token_kind {
    vu_NoToken,
    vu_Keyword,
    vu_NumLiteral
};


typedef struct {
    enum vu_Token_kind kind;

    // Starting pointer into source
    char* start;

    // End pointer into source
    char* end;
} vu_Token_t;


vu_Token_t newToken();


#endif