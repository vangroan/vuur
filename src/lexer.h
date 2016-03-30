

#ifndef VUUR_LEXER
#define VUUR_LEXER

#include "scanner.h"
#include "token.h"
#include "types.h"

typedef struct {
    vuScanner* scanner;
    vu_Bool done;
} vu_Lexer_t;


vu_Lexer_t* vu_Lexer_new(vuScanner* scanner);
void vu_Lexer_free(vu_Lexer_t* self);
vu_Token_t* vu_Lexer_scan(vu_Lexer_t* self);

#endif