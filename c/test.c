#include <math.h>
#include <stdio.h>

#include "png_achunk.h"

int main() {
  char *bytes;
  size_t length;

  read_chunk("/tmp/test.png", "teST", &bytes, &length);

  if (bytes) {
    for (int i = 0; i < length; i++) {
      printf("%x\n", bytes[i]);
    }
    free_chunk(bytes, length);
  }
  return 0;
}
