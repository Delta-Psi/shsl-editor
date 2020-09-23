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
    const QHash<QString, int>& dirs() const
    {
        return dirMap;
    }

    bool containsFile(const QString& path) const
    {
        return fileMap.contains(path);
    }
    QByteArray readFile(const QString& path);

private:
    QFile handle;

    quint64 headerSize;

    QVector<File> fileList;
    QHash<QString, int> fileMap;

    QVector<Dir> dirList;
    QHash<QString, int> dirMap;
};

#endif // WAD_H
