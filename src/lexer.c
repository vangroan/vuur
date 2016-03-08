
#include "lexer.h"


vu_Lexer_t* vu_Lexer_new(vu_scanner_t* scanner) {
    vu_Lexer_t* lexer = (vu_Lexer_t*)malloc(sizeof(vu_Lexer_t));
    lexer->scanner = scanner;
    lexer->done = vu_False;
    return lexer;
}

void vu_Lexer_free(vu_Lexer_t* self) {
    free(self);
}

vu_Token_t* vu_Lexer_scan(vu_Lexer_t* self) {
    return NULL;
}