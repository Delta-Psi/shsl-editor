#ifndef WADFILES_H
#define WADFILES_H

#include <QAbstractItemModel>
#include "wad.h"

class WadFilesModel : public QAbstractItemModel
{
    Q_OBJECT
public:
    WadFilesModel(const Wad *wad=nullptr);

    void setWad(const Wad *wad);

public slots:
    void clear();

    // QAbstractItemModel interface
public:
    QModelIndex index(int row, int column, const QModelIndex &parent) const override;
    QModelIndex parent(const QModelIndex &child) const override;
    int rowCount(const QModelIndex &parent) const override;
    int columnCount(const QModelIndex &parent) const override;
    QVariant data(const QModelIndex &index, int role) const override;

private:
    const Wad *_wad;

    struct Entry
    {
        int row;
        int parent;

        QString name;
        bool directory, file;
        int index;
        quint64 offset, size;

        QVector<int> children;
    };
    QVector<Entry> entries;
    void updateEntries();
    void updateEntriesSub(int parentId, const QString &parentPath);
};

#endif // WADFILES_H
