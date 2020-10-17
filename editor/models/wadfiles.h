#ifndef WADFILES_H
#define WADFILES_H

#include <QAbstractItemModel>
#include "project.h"
#include "wad.h"

class WadFilesModel : public QAbstractItemModel
{
    Q_OBJECT
public:
    WadFilesModel();

    void setFiles(GameFiles *files);
    bool canReadEntry(const QModelIndex &index);
    QByteArray readEntry(const QModelIndex &index);

    void onRightClick(const QModelIndex &index, QWidget *menuParent);

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
    GameFiles *files;

    struct Entry
    {
        int row;
        int parent;

        QString name;
        bool directory, file;
        bool us;
        int index;
        //quint64 offset, size;

        QVector<int> children;
    };
    QVector<Entry> entries;
    void updateEntries();
    void updateEntriesSub(int parentId, const QString &parentPath);

    QByteArray readEntry(const Entry &entry);
};

#endif // WADFILES_H
