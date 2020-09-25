#ifndef WAD_H
#define WAD_H

#include <QFile>
#include <QHash>

class Wad
{
public:
    struct File
    {
        QString path;
        quint64 size;
        quint64 offset;

        File(QString path, quint64 size, quint64 offset):
            path(path), size(size), offset(offset) {}
    };

    struct Dir
    {
        struct Subfile
        {
            QString name;
            bool isDirectory;

            Subfile(QString name, bool isDirectory):
                name(name), isDirectory(isDirectory) {}
        };

        QString path;
        QVector<Subfile> subfiles;

        Dir(QString path):
            path(path) {}
    };

    Wad(const QString& path);

    const QHash<QString, int>& files() const
    {
        return fileMap;
    }

    int fileIndex(const QString& path) const
    {
        return fileMap.value(path, -1);
    }
    quint64 fileSize(int index) const
    {
        return fileList[index].size;
    }
    QByteArray readFile(int index);

private:
    QFile handle;

    quint64 headerSize;

    QVector<File> fileList;
    QHash<QString, int> fileMap;

    QVector<Dir> dirList;
    QHash<QString, int> dirMap;
};

#endif // WAD_H
