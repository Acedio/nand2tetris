%option noyywrap
%{
#include <stdio.h>
#include "hack.tab.h"

int line_no = 1;
%}

%s AT_STATE

SYMBOL_LEAD [A-Za-z_.$:]
SYMBOL_CHAR [A-Za-z_.$:0-9]
%%
\/\/.*/\n ;
[ \t] ;
\n { ++line_no; }

@ { BEGIN(AT_STATE); return yytext[0]; }
<AT_STATE>[0-9]+ { BEGIN(INITIAL); yylval.num = atoi(yytext); return INT; }
=|;|\(|\) { return yytext[0]; }

SP     { BEGIN(INITIAL); yylval.num = 0; return INT; }
LCL    { BEGIN(INITIAL); yylval.num = 1; return INT; }
ARG    { BEGIN(INITIAL); yylval.num = 2; return INT; }
THIS   { BEGIN(INITIAL); yylval.num = 3; return INT; }
THAT   { BEGIN(INITIAL); yylval.num = 4; return INT; }
R0     { BEGIN(INITIAL); yylval.num = 0; return INT; }
R1     { BEGIN(INITIAL); yylval.num = 1; return INT; }
R2     { BEGIN(INITIAL); yylval.num = 2; return INT; }
R3     { BEGIN(INITIAL); yylval.num = 3; return INT; }
R4     { BEGIN(INITIAL); yylval.num = 4; return INT; }
R5     { BEGIN(INITIAL); yylval.num = 5; return INT; }
R6     { BEGIN(INITIAL); yylval.num = 6; return INT; }
R7     { BEGIN(INITIAL); yylval.num = 7; return INT; }
R8     { BEGIN(INITIAL); yylval.num = 8; return INT; }
R9     { BEGIN(INITIAL); yylval.num = 9; return INT; }
R10    { BEGIN(INITIAL); yylval.num = 10; return INT; }
R11    { BEGIN(INITIAL); yylval.num = 11; return INT; }
R12    { BEGIN(INITIAL); yylval.num = 12; return INT; }
R13    { BEGIN(INITIAL); yylval.num = 13; return INT; }
R14    { BEGIN(INITIAL); yylval.num = 14; return INT; }
R15    { BEGIN(INITIAL); yylval.num = 15; return INT; }
SCREEN { BEGIN(INITIAL); yylval.num = 16384; return INT; }
KBD    { BEGIN(INITIAL); yylval.num = 24576; return INT; }

0    { yylval.bin = "0101010"; return COMP; }
1    { yylval.bin = "0111111"; return COMP; }
-1   { yylval.bin = "0111010"; return COMP; }
D    { yylval.bin = "0001100"; return COMP; }
A    { yylval.bin = "0110000"; return COMP; }
!D   { yylval.bin = "0001101"; return COMP; }
!A   { yylval.bin = "0110001"; return COMP; }
-D   { yylval.bin = "0001111"; return COMP; }
-A   { yylval.bin = "0110011"; return COMP; }
D\+1 { yylval.bin = "0011111"; return COMP; }
A\+1 { yylval.bin = "0110111"; return COMP; }
D-1  { yylval.bin = "0001110"; return COMP; }
A-1  { yylval.bin = "0110010"; return COMP; }
D\+A { yylval.bin = "0000010"; return COMP; }
D-A  { yylval.bin = "0010011"; return COMP; }
A-D  { yylval.bin = "0000111"; return COMP; }
D&A  { yylval.bin = "0000000"; return COMP; }
D\|A { yylval.bin = "0010101"; return COMP; }
M    { yylval.bin = "1110000"; return COMP; }
!M   { yylval.bin = "1110001"; return COMP; }
-M   { yylval.bin = "1110011"; return COMP; }
M\+1 { yylval.bin = "1110111"; return COMP; }
M-1  { yylval.bin = "1110010"; return COMP; }
D\+M { yylval.bin = "1000010"; return COMP; }
D-M  { yylval.bin = "1010011"; return COMP; }
M-D  { yylval.bin = "1000111"; return COMP; }
D&M  { yylval.bin = "1000000"; return COMP; }
D\|M { yylval.bin = "1010101"; return COMP; }

A/=   { yylval.bin = "100"; return DEST; }
D/=   { yylval.bin = "010"; return DEST; }
M/=   { yylval.bin = "001"; return DEST; }
AD/=  { yylval.bin = "110"; return DEST; }
AM/=  { yylval.bin = "101"; return DEST; }
MD/=  { yylval.bin = "011"; return DEST; }
AMD/= { yylval.bin = "111"; return DEST; }

JGT { yylval.bin = "001"; return JUMP; }
JEQ { yylval.bin = "010"; return JUMP; }
JGE { yylval.bin = "011"; return JUMP; }
JLT { yylval.bin = "100"; return JUMP; }
JNE { yylval.bin = "101"; return JUMP; }
JLE { yylval.bin = "110"; return JUMP; }
JMP { yylval.bin = "111"; return JUMP; }

{SYMBOL_LEAD}{SYMBOL_CHAR}* { BEGIN(INITIAL); // just in case we're in AT_STATE
                              yylval.str = strdup(yytext);
                              return SYMBOL; }
%%
