%{
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int yylex();
void yyerror(char* s);

extern int line_no;
int cur_address = 0;
int label_no = 0;

typedef struct {
  char* label;
  int address;
} Label;

#define MAX_LABELS 20000
Label labels[MAX_LABELS];

// return address of label or -1 if it doesn't exist
int get_label(char* label) {
  int l = 0;
  for (; l < label_no; ++l) {
    if (strcmp(label, labels[l].label) == 0) {
      return labels[l].address;
    }
  }
  return -1;
}

// returns index of label
int add_label(char* label, int address) {
  int l = 0;
  for (; l < label_no; ++l) {
    if (strcmp(label, labels[l].label) == 0) {
      return l;
    }
  }
  labels[label_no].label = strdup(label);
  labels[label_no].address = address;
  ++label_no;
  return label_no - 1;
}

void clear_labels() {
  int l = 0;
  for (; l < label_no; ++l) {
    if (labels[l].label) {
      free(labels[l].label);
    }
  }
  label_no = 0;
}

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
    lines { printf("Labels:\n");
            int l = 0;
            for (; l < label_no; ++l) {
              printf(" %d: %s = %d\n", l, labels[l].label, labels[l].address);
            }
            clear_labels();
          }
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
         COMP ';' JUMP { printf("111%s000%s\n", $1, $3); ++cur_address; }
         | DEST '=' COMP { printf("111%s%s000\n", $3, $1); ++cur_address; }
         | DEST '=' COMP ';' JUMP { printf("111%s%s%s\n", $3, $1, $5); ++cur_address; }
         ;
a_command:
         '@' INT { printf("%016d\n", $2); ++cur_address; }
         | '@' SYMBOL { printf("found a symbol %s on line %d that we can't do anything with yet.\n", $2, line_no); }
         ;
label:
     '(' SYMBOL ')' { if (get_label($2) >= 0) {
                        yyerror("label already exists");
                      }
                      add_label($2, cur_address); }
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
