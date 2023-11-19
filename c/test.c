#include <math.h>
#include <stdio.h>

void read_chunk(const char *filename, const char *name, char **bytes,
                size_t *length);
void free_chunk(char *bytes, size_t length);

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
