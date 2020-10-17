#ifndef HELPER_H
#define HELPER_H

#include "../tga/tga.h"

// used so we can read tga files from memory
// does NOT take ownership of the data
class MemoryInterface : public tga::FileInterface
{
public:
    MemoryInterface(uint8_t* buffer, size_t size);
    ~MemoryInterface() {}

    // Returns true if we can read/write bytes from/into the file
    bool ok() const;

    // Current position in the file
    size_t tell();

    // Jump to the given position in the file
    void seek(size_t absPos);

    // Returns the next byte in the file or 0 if ok() = false
    uint8_t read8();

    // Writes one byte in the file (or do nothing if ok() = false)
    void write8(uint8_t value);

private:
    uint8_t *buffer;
    size_t size;
    size_t position;
};

#endif // HELPER_H
