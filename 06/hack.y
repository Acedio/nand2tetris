%{
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int yylex();
void yyerror(char* s);

extern int line_no;

#define LINE_SIZE 17

#define MAX_ADDRESS 100000
char* rom[MAX_ADDRESS];
int cur_address = 0;

typedef struct {
  char* label;
  int address;
} Label;

#define MAX_LABELS 20000
Label labels[MAX_LABELS];
int label_no = 0;

#define MAX_LOOKBACKS 20000
Label lookbacks[MAX_LOOKBACKS];
int lookback_no = 0;

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

// Takes ownership of instruction.
void add_instruction(char* instruction) {
  if (cur_address >= MAX_ADDRESS) {
    yyerror("Max address reached.");
  }
  rom[cur_address] = instruction;
}

void print_program() {
  int a = 0;
  for (; a < cur_address; ++a) {
    puts(rom[a]);
  }
}

void clear_rom() {
  int a = 0;
  for (; a < cur_address; ++a) {
    if (rom[a]) {
      free(rom[a]);
    }
  }
}

char* create_a(int address) {
  char* new_line = (char*)malloc((sizeof(char)) * LINE_SIZE);
  new_line[16] = 0;
  int bit = 15;
  for (; bit >= 0; --bit) {
    new_line[bit] = (address & 1) ? '1' : '0';
    address >>= 1;
  }
  return new_line;
}

char* create_c(char* comp, char* dest, char* jump) {
  if (!comp) yyerror("MISSING COMP");
  if (!dest) dest = "000";
  if (!jump) jump = "000";
  char* new_line = (char*)malloc((sizeof(char)) * LINE_SIZE);
  snprintf(new_line, LINE_SIZE, "111%s%s%s", comp, dest, jump);
  return new_line;
}

void add_lookback(char* label, int address) {
  lookbacks[lookback_no].label = strdup(label);
  lookbacks[lookback_no].address = address;
  ++lookback_no;
}

void fix_lookbacks() {
  int next_free = 16;  // TODO: need to check for overflowing the RAMs bounds
  int l = 0;
  for(; l < lookback_no; ++l) {
    int address = get_label(lookbacks[l].label);
    if (address >= 0) {
      // if the label exists, use it
      // rom takes ownership
      rom[lookbacks[l].address] = create_a(address);
    } else {
      // if the label doesn't exist, create it
      address = next_free;
      ++next_free;

      add_label(lookbacks[l].label, address);

      // rom takes ownership
      rom[lookbacks[l].address] = create_a(address);
    }
  }
}

void clear_lookbacks() {
  int l = 0;
  for (; l < lookback_no; ++l) {
    if (lookbacks[lookback_no].label) {
      free(lookbacks[lookback_no].label);
    }
  }
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
    lines { fprintf(stderr, "Labels:\n");
            int l = 0;
            for (; l < label_no; ++l) {
              fprintf(stderr, " %d: %s = %d\n", l, labels[l].label, labels[l].address);
            }
            fix_lookbacks();
            print_program();
            clear_labels();
            clear_lookbacks();
            clear_rom();
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
         COMP ';' JUMP { rom[cur_address] = create_c($1, 0, $3); ++cur_address; }
         | DEST '=' COMP { rom[cur_address] = create_c($3, $1, 0); ++cur_address; }
         | DEST '=' COMP ';' JUMP { rom[cur_address] = create_c($3, $1, $5); ++cur_address; }
         ;
a_command:
         '@' INT { rom[cur_address] = create_a($2); ++cur_address; }
         | '@' SYMBOL { rom[cur_address] = NULL; add_lookback($2, cur_address); ++cur_address; }
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
