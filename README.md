# pngme

https://picklenerd.github.io/pngme_book/

## Assignment 1: Chunk Types

What I've learned so far.
A png file consists out of:
- a png signature
- followed by a series of chunks

The png signature consists out of 8 bytes always containing the same (decimal) values: `137 80 78 71 13 10 26 10`.
There are different types of chunks. The series of chunks begin with an IHDR chunk and end with an IEND chunk.

Each chunk consists out of 4 parts:
1. Length (4 bytes / 32 bit) - the amount of bytes in the chunk data.
2. Chunk Type (4 bytes / 32 bit) - the type of the chunk.
3. Chunk Data (variable)
4. CRC (4 bytes / 32 bit)
