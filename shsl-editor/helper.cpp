#include "helper.h"

MemoryInterface::MemoryInterface(uint8_t *buffer, size_t size)
    : buffer(buffer)
    , size(size)
    , position(0)
{
}

bool MemoryInterface::ok() const
{
    return position < size;
}

size_t MemoryInterface::tell()
{
    return position;
}

void MemoryInterface::seek(size_t absPos)
{
    if (absPos < size) position = absPos;
}

uint8_t MemoryInterface::read8()
{
    return buffer[position++];
}

void MemoryInterface::write8(uint8_t value)
{
    if (position < size) {
        buffer[position] = value;
        ++position;
    }
}
