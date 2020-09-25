#include "error.h"
#include "wad.h"

#include <QDataStream>
#include <QDebug>

Wad::Wad(const QString &path):
    handle(path)
{
    if (!handle.open(QIODevice::ReadOnly))
    {
        throw Error("Unable to open file");
    }

    QDataStream stream(&handle);
    stream.setByteOrder(QDataStream::LittleEndian);

    // read magic numbers
    QByteArray buffer;
    buffer.resize(4);
    stream.readRawData(buffer.data(), buffer.size());
    if (buffer != "AGAR")
    {
        throw Error("Invalid WAD file.");
    }

    // read version
    quint32 versionMajor = 0, versionMinor = 0;
    stream >> versionMajor >> versionMinor;

    // disregard these bytes
    stream.skipRawData(4);

    headerSize = 16;

    // read file metadata
    quint32 fileCount = 0;
    stream >> fileCount; headerSize += 4;
    fileList.reserve(fileCount);

    for(quint32 i = 0; i < fileCount; ++i)
    {
        quint32 pathSize = 0;
        stream >> pathSize; headerSize += 4;

        QByteArray pathData;
        pathData.resize(pathSize);
        stream.readRawData(pathData.data(), pathData.size()); headerSize += pathSize;
        QString path(pathData);

        quint64 size = 0;
        stream >> size; headerSize += 8;
        quint64 offset = 0;
        stream >> offset; headerSize += 8;

        fileList.append(File(path, size, offset));
        fileMap.insert(path, i);
    }

    // read directory metadata
    quint32 dirCount = 0;
    stream >> dirCount; headerSize += 4;
    dirList.reserve(dirCount);

    for(quint32 i = 0; i < dirCount; ++i)
    {
        quint32 pathSize = 0;
        stream >> pathSize; headerSize += 4;

        QByteArray pathData;
        pathData.resize(pathSize);
        stream.readRawData(pathData.data(), pathData.size()); headerSize += pathSize;
        QString path(pathData);

        dirList.append(Dir(path));
        dirMap.insert(path, i);
        Dir& dir = dirList[i];

        quint32 subfileCount = 0;
        stream >> subfileCount; headerSize += 4;
        dir.subfiles.reserve(subfileCount);

        for(quint32 j = 0; j < subfileCount; ++j)
        {
            quint32 nameSize = 0;
            stream >> nameSize; headerSize += 4;

            QByteArray nameData;
            nameData.resize(nameSize);
            stream.readRawData(nameData.data(), nameData.size()); headerSize += nameSize;
            QString name(nameData);

            quint8 isDirectory = false;
            stream >> isDirectory; headerSize += 1;

            dir.subfiles.append(Dir::Subfile(name, isDirectory != 0));
        }
    }
}

QByteArray Wad::readFile(int index)
{
    const File& file = fileList.at(index);

    quint64 offset = headerSize + file.offset;
    handle.seek(offset);

    QByteArray buffer;
    buffer.resize(file.size);
    qint64 readBytes = handle.read(buffer.data(), buffer.size());
    Q_ASSERT(readBytes >= 0);
    Q_ASSERT((quint64)readBytes == file.size);

    return buffer;
}
