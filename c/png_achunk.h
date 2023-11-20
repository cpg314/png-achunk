#include <stdio.h>

#ifndef PNGACHUNK_H_
#define PNGACHUNK_H_

// Read the chunk with type `name` in the file `filename`.
// If an errors occurs, `bytes` is set to a null pointer.
void read_chunk(const char *filename, const char *name, char **bytes,
                size_t *length);
// Free the memory taken by a chunk returned by `read_chunk`.
void free_chunk(char *bytes, size_t length);

#endif // PNGACHUNK_H_
