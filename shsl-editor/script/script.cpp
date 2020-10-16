#include "script.h"

#include <QtEndian>

bool Script::decode(const QByteArray &data)
{
    // read script type (1 for no strings, 2 for strings)
    if (data.size() < 4) return false;
    auto type = qFromLittleEndian<quint32>(data.constData());

    // read instruction offset
    if (data.size() < 4+4) return false;
    auto instructionsOffset = qFromLittleEndian<quint32>(data.constData() + 4);
    if (!readInstructions(data.mid(instructionsOffset))) return false;

    if (type == 2) {
        // read string offset
        if (data.size() < 8+4) return false;
        auto stringsOffset = qFromLittleEndian<quint32>(data.constData() + 8);
        if (!readStrings(data.mid(stringsOffset))) return false;
    }

    return true;
}

bool Script::readInstructions(const QByteArray &data)
{
    // TODO
    return true;
}

bool Script::readStrings(const QByteArray &data)
{
    quint32 count;
    qFromLittleEndian<quint32>(data.data(), 1, &count);

    for (quint32 i = 0; i < count; ++i) {
        quint32 stringOffset;
        qFromLittleEndian<quint32>(data.data() + 4 + i*4, 1, &stringOffset);

        QString string = QString::fromUtf16((const ushort*)(data.constData() + stringOffset));
        strings.append(string);
    }

    return true;
}
