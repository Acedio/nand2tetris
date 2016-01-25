#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <tuple>
#include <vector>

enum class TileType {
  UNKNOWN = -1,
  FLOOR = 0,
  WALL = 1,
  INVERTILE = 2,
  MIRROR = 3,
  WHITE = 4,
  DOOR = 5,
  SECRET = 6
};

struct Tile {
  int x, y;
  TileType type;
};

bool operator==(const Tile& lhs, const Tile& rhs) {
  return std::tie(lhs.x, lhs.y, lhs.type) == std::tie(rhs.x, rhs.y, rhs.type);
}

TileType char_to_type(char c) {
  switch(c) {
    case ' ':
      return TileType::FLOOR;
    case '*':
      return TileType::DOOR;
    case '#':
      return TileType::WALL;
    case '!':
      return TileType::INVERTILE;
    case 'm':
      return TileType::MIRROR;
    case 'w':
      return TileType::WHITE;
    case 's':
      return TileType::SECRET;
  }
  return TileType::UNKNOWN;
}

struct Link {
  int x, y;
  int next_level;
  int nx, ny;
};

struct Level {
  int number;
  int w, h;
  const char* map;
  std::vector<Link> links;
};

int format_link(const Link& link) {
  // 1 and 15 rather than 0 and 16 because we want to make sure we warp within
  // the walls
  assert(link.nx >= 1);
  assert(link.nx < 15);
  assert(link.ny >= 1);
  assert(link.ny < 15);
  assert(link.next_level >= 0);
  assert(link.next_level < 128);
  return 0x8000 | (link.nx << 11) | (link.ny << 7) | link.next_level;
}

void print_level(const Level& level) {
  // TODO: w and h must be 16 right now
  assert(level.h == 16);
  assert(level.w == 16);
  printf("    if (num = %d) {\n", level.number);
  //printf("      let w = %d;\n", level.w);
  //printf("      let h = %d;\n", level.h);

  std::vector<Tile> specials;
  std::vector<Tile> doors;  // for checking later
  int ci = 0;
  for (int y = 0; y < level.h; ++y) {
    unsigned int row = 0;
    unsigned int mask = 1;
    for (int x = 0; x < level.w; ++x) {
      TileType type = char_to_type(level.map[ci]);
      if (type == TileType::WALL) {
        row |= mask;
      } else if (type == TileType::FLOOR) {
        // nada, should be a 0
      } else if (type == TileType::DOOR) {
        // doors can be walked through, so we have them default to 0, but it
        // shouldn't matter
        doors.push_back({x, y, type});
      } else {  // special tile
        row |= mask;  // wall by default but shouldn't matter
        // save to print later
        specials.push_back({x, y, type});
      }
      mask <<= 1;
      ++ci;
    }
    row &= 0xFFFF;
    if (row > 0x7FFF) {
      // jack can't handle large 16 bit integers, so invert a negative one
      printf("      let rows[%d] = ~%d;\n", y, (~row) & 0x7FFF);
    } else {
      printf("      let rows[%d] = %d;\n", y, row & 0x7FFF);
    }
  }
  printf("      do map.load_rows(rows);\n");
  for (const Tile t : specials) {
    if (t.type == TileType::SECRET) {
      // Print the secret (0x4000 with the flag for this room)
      printf("      do map.set_tile(%d, %d, %d);\n", t.x, t.y,
             (1 << 14) | (1 << level.number));
    } else {
      printf("      do map.set_tile(%d, %d, %d);\n", t.x, t.y, t.type);
    }
  }
  bool link_error = false;
  std::vector<Link> links = level.links;
  while (!links.empty()) {
    Link link = links.back();
    links.pop_back();
    Tile door = {link.x, link.y, TileType::DOOR};
    auto new_end = std::remove(doors.begin(), doors.end(), door);
    if (new_end == doors.end()) {
      fprintf(stderr,
              "WTF: couldn't find door to match link with x = %d y = %d\n",
              link.x, link.y);
      link_error = true;
    } else {
      doors.pop_back();
      assert(doors.end() == new_end);
    }

    int formatted_link = format_link(link);

    if (formatted_link > 0x7FFF) {
      printf("      do map.set_tile(%d, %d, ~%d);\n", link.x, link.y,
             (~formatted_link) & 0x7FFF);
    } else {
      printf("      do map.set_tile(%d, %d, %d);\n", link.x, link.y,
             formatted_link);
    }
  }
  if (!doors.empty()) {
    link_error = true;
    for (auto door : doors) {
      fprintf(stderr,
              "WTF: link not found that matches door at x = %d y = %d\n",
              door.x, door.y);
    }
  }
  assert(!link_error);
  printf("    }\n");
}

void print_levels() {
  printf("  function void load_level(Map map, int num) {\n");
  printf("    var Array rows;\n");
  printf("    let rows = Array.new(16);\n");
  
  /*
  // Template
  print_level({0, 16, 16,
              "################"
              "#              #"
              "#              #"
              "#              #"
              "#              #"
              "#              #"
              "#              #"
              "#              #"
              "#              #"
              "#              #"
              "#              #"
              "#              #"
              "#              #"
              "#              #"
              "#              #"
              "################",
              {
                // links go here
              }});
  */

  print_level({0, 16, 16,
              "############*###"
              "#     s#       #"
              "#      #       #"
              "#      #       #"
              "###  ######  ###"
              "# #  #    #  # #"
              "# #  #    #  # #"
              "###  ######  # #"
              "#        ##  # #"
              "#            # #"
              "#            # #"
              "#        ##  # #"
              "#        ##  ###"
              "#        ##    *"
              "#        ##    #"
              "####*###########",
              {
                {12, 0, 1, 12, 14},
                {15, 13, 2, 1, 13},
                {4, 15, 3, 1, 1}
              }});

  print_level({1, 16, 16,
              "################"
              "######m##m##m###"
              "#####          #"
              "#              *"
              "#  ##          #"
              "#  ###m##m##m###"
              "#  #############"
              "#            m #"
              "#            m #"
              "####  #####  m #"
              "####  ###s#  m #"
              "#         #  # #"
              "#  #  #  ##  ###"
              "#        #     #"
              "#        #     #"
              "#mmmmmmmm###*###",
              {
                {12, 15, 0, 12, 1},
                {15, 3, 2, 1, 4}
              }});

  print_level({2, 16, 16,
              "################"
              "#              #"
              "# #  #  #  #  w#"
              "#              #"
              "*             s#"
              "#              #"
              "# #  #  #  #  w#"
              "#              #"
              "#!!!!!!!!!######"
              "#         #    #"
              "#         #    #"
              "#######  ##    #"
              "#######  #     #"
              "*        #     #"
              "#        #     #"
              "################",
              {
                {0, 13, 0, 14, 13},
                {0, 4, 1, 14, 3}
              }});

  print_level({3, 16, 16,
              /* Break in case frustration is fun for you.
              "#*##############"
              "# #     #     ##"
              "# # # # # ### ##"
              "#   # # #   # ##"
              "####### ### # ##"
              "#   #       # ##"
              "# # # ####### ##"
              "# #   #       ##"
              "# ##### ##### ##"
              "#   #   #   #m##"
              "### # ### # ####"
              "#   # #   #   ##"
              "# ##### ### # ##"
              "#       #   #s##"
              "################"
              "################",
              */
              "#*###########mm#"
              "#  #        #  #"
              "#  #        #  #"
              "#  #  ####  #  #"
              "#     m        #"
              "#     m        #"
              "#!!#######  ####"
              "#     #     #ss#"
              "#     #     #ss#"
              "#  #  #  #!!#  #"
              "#  #  #     #  #"
              "#  #  #     #  #"
              "#  ####  #  #  #"
              "#        #     #"
              "#        #     #"
              "################",
              {
                {1, 0, 0, 4, 14}
              }});

  printf("    do rows.dispose();\n");
  printf("    return;\n");
  printf("  }\n");
}

void print_bits() {
  printf("  function void init_bits(Array lut) {\n");
  int i = 0;
  int values = 15;
  for (i = 0; i < values; ++i) {
    printf("    let lut[%d] = %d;\n", i, 1 << i);
  }
  // special case 15
  printf("    let lut[15] = ~32767;\n");
  printf("    return;\n  }\n\n");
}

void print_delta_distance() {
  printf("  function void init_delta_dist(Array lut, Array low_bits) {\n");
  int i = 0;
  int values = 256;  // number of discrete angles
  for (i = 0; i < values; ++i) {
    // offset by 0.5 to avoid issues with divide by zero.
    double theta = ((i + 0.5) * (2.0 * 3.14159)) / values;
    double value = sqrt(1.0 + (sin(theta) * sin(theta)) / (cos(theta) * cos(theta)));
    int int_val = 256 * value;
    printf("    let lut[%d] = %d;\n", i, int_val & 0xFFFF);
    // We want to know where the 7 most important bits are so we can multiply
    // with higher precision
    int hob = 0;
    while (int_val >>= 1) {
      ++hob;
    }
    printf("    let low_bits[%d] = %d;\n", i, hob - 6);
  }
  printf("    return;\n  }\n\n");
}

void print_x_angle() {
  printf("  function void init_x_angle(Array LUT_high, Array LUT_low) {\n");
  // assume a 90 degree FOV, so camera plane is twice the (unit) camera direction
  double camera_plane_w = 2.0;
  // caller should also start at -32
  int last_int_theta = -32 * 256;
  int values = 32;  // number of columns on the screen
  int i;
  for (i = 31; i >= 0; --i) {
    double theta = atan2(camera_plane_w * (values / 2 - i) / (float)values, 1.0);
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

void print_height_from_dist() {
  printf("  function void init_height_from_dist(Array lut) {\n");
  int i = 0;
  int values = 256;  // furthest possible dist (eh)
  for (i = 0; i < values; ++i) {
    double height = 32.0 * 130.0 / (i + 1);
    if (height > 256) height = 256;
    double from_top = (256.0 - height) / 2;
    int int_val = from_top;
    printf("    let lut[%d] = %d;\n", i, int_val);
  }
  printf("    return;\n  }\n\n");
}

void print_cos() {
  printf("  function void init_cos(Array lut) {\n");
  int i = 0;
  int values = 256;  // furthest possible dist (eh)
  for (i = 0; i < values; ++i) {
    double theta = ((i + 0.5) * (2.0 * 3.14159)) / values;
    int int_val = 30 * cos(theta);
    printf("    let lut[%d] = %d;\n", i, int_val);
  }
  printf("    return;\n  }\n\n");
}

int main() {
  printf("class Luts {\n");
  print_bits();
  print_delta_distance();
  print_x_angle();
  print_height_from_dist();
  print_cos();
  print_levels();
  printf("}\n");
  return 0;
}
