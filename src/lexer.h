

#ifndef VUUR_LEXER
#define VUUR_LEXER

#include <stdbool.h>

#include "scanner.h"
#include "token.h"

typedef struct {
    struct VuScanner* scanner;
    bool done;
} vu_Lexer_t;


vu_Lexer_t* vu_Lexer_new(struct VuScanner* scanner);
void vu_Lexer_free(vu_Lexer_t* self);
vu_Token_t* vu_Lexer_scan(vu_Lexer_t* self);

#endif