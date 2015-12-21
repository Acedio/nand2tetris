%{
#include <stdio.h>

int yylex();
void yyerror(char* s);

extern int line_no;
%}

%union {
   int num;
   char *str;
   char *bin;
}

%token <str> COMP
%token <str> DEST
%token <str> JUMP
%token <str> SYMBOL
%token <num> INT

%%

hack:
    lines
    ;
lines:
     lines line
     | line
     ;
line:
    c_command
    | a_command
    | label
    ;
c_command:
         COMP ';' JUMP { printf("111%s000%s\n", $1, $3); }
         | DEST '=' COMP { printf("111%s%s000\n", $3, $1); }
         | DEST '=' COMP ';' JUMP { printf("111%s%s%s\n", $3, $1, $5); }
         ;
a_command:
         '@' INT { printf("%016d\n", $2); }
         | '@' SYMBOL { printf("found a symbol %s on line %d that we can't do anything with yet.\n", $2, line_no); }
         ;
label:
     '(' SYMBOL ')' { printf("found a label %s on line %d that we can't do much with yet.\n", $2, line_no); }
     ;

%%

int main(int argc, char** argv) {
  yyparse();
  return 0;
}

void yyerror(char* s) {
  fprintf(stderr, "PARSE ERROR on line %d: %s\n", line_no, s);
  exit(-1);
}
