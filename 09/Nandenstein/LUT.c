#include <stdio.h>
#include <math.h>

int main() {
  int i = 0;
  int values = 256;
  for (i = 0; i < values; ++i) {
    // offset by 0.5 to avoid issues with divide by zero.
    double theta = ((i + 0.5) * (2.0 * 3.14159)) / values;
    double value = sqrt(1.0 + (sin(theta) * sin(theta)) / (cos(theta) * cos(theta)));
    int int_val = 256 * value;
    printf("let LUT_high[%d] = %d;\n", i, int_val >> 8);
    printf("let LUT_low[%d] = %d;\n", i, int_val & 0xFF);
  }
  return 0;
}
