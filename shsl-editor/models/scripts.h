#ifndef SCRIPTS_H
#define SCRIPTS_H

#include <QAbstractItemModel>

#include "project.h"
#include "wad.h"

class ScriptsModel : public QAbstractItemModel
{
public:
    ScriptsModel();

    void setFiles(GameFiles *files);

private:
    Wad *wad;

    struct Entry {
        QString name;
        int index;

        Entry(QString name, int index): name(name), index(index) {}
    };
    QVector<Entry> entries;
    void updateEntries();

    // QAbstractItemModel interface
public:
    QModelIndex index(int row, int column, const QModelIndex &parent) const override;
    QModelIndex parent(const QModelIndex &child) const override;
    int rowCount(const QModelIndex &parent) const override;
    int columnCount(const QModelIndex &parent) const override;
    QVariant data(const QModelIndex &index, int role) const override;
};

#endif // SCRIPTS_H
