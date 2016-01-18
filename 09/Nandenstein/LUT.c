#include <stdio.h>
#include <math.h>

void print_delta_distance() {
  printf("  function void init_delta_dist(Array LUT_high, Array LUT_low) {\n");
  int i = 0;
  int values = 256;  // number of discrete angles
  for (i = 0; i < values; ++i) {
    // offset by 0.5 to avoid issues with divide by zero.
    double theta = ((i + 0.5) * (2.0 * 3.14159)) / values;
    double value = sqrt(1.0 + (sin(theta) * sin(theta)) / (cos(theta) * cos(theta)));
    int int_val = 256 * value;
    printf("    let LUT_high[%d] = %d;\n", i, int_val >> 8);
    printf("    let LUT_low[%d] = %d;\n", i, int_val & 0xFF);
  }
  printf("    return;\n  }\n\n");
}

void print_x_angle() {
  printf("  function void init_x_angle(Array LUT_high, Array LUT_low) {\n");
  // assume a 90 degree FOV, so camera plane is twice the (unit) camera direction
  double camera_plane_w = 2.0;
  // caller should also start at -32
  int last_int_theta = -32 * 256;
  int i = 0;
  int values = 32;  // number of columns on the screen
  for (i = 0; i < values; ++i) {
    double theta = atan2(camera_plane_w * (i - values / 2) / (float)values, 1.0);
    // transform from a 2*pi circle to a 256 circle
    double theta_256 = 256 * theta / (2 * 3.14159);
    int int_theta = 256 * theta_256;
    int int_val = int_theta - last_int_theta;
    last_int_theta = int_theta;
    printf("    let LUT_high[%d] = %d;\n", i, int_val >> 8);
    printf("    let LUT_low[%d] = %d;\n", i, int_val & 0xFF);
  }
  printf("    return;\n  }\n\n");
}

int main() {
  printf("class Luts {\n");
  print_delta_distance();
  print_x_angle();
  printf("}\n");
  return 0;
}
