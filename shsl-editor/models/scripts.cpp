#include "scripts.h"

#include <QLocale>

ScriptsModel::ScriptsModel():
    wad(nullptr)
{
}

void ScriptsModel::setFiles(GameFiles *files)
{
    wad = files->get(GameFiles::DR2_DATA_US);
    updateEntries();
}

QByteArray ScriptsModel::readEntry(const QModelIndex &index)
{
    return wad->readFile(entries[index.internalId()].index);
}

void ScriptsModel::updateEntries()
{
    beginResetModel();

    entries.clear();
    if (wad)
    {
        const QString prefix = "Dr2/data/us/script";
        int dirIndex = wad->dirIndex(prefix);
        const QVector<Wad::Dir::Subfile> &subfiles = wad->dirSubfiles(dirIndex);

        for(const Wad::Dir::Subfile &subfile: subfiles)
        {
            if (subfile.isDirectory) continue;

            // check if the subfile name matches e??_???_???.lin
            if (subfile.name.size() != 15) continue;
            if (subfile.name[0] != 'e') continue;
            if (!subfile.name.endsWith(".lin")) continue;

            int fileIndex = wad->fileIndex(prefix + "/" + subfile.name);
            Q_ASSERT(fileIndex != -1);
            entries.append(Entry(subfile.name, fileIndex));
        }
    }

    // ensure the scripts are listed alphabetically
    std::sort(entries.begin(),
          entries.end(),
          [](const Entry &a, const Entry &b)->bool{return a.name < b.name;});

    endResetModel();
}

QModelIndex ScriptsModel::index(int row, int column, const QModelIndex &parent) const
{
    Q_UNUSED(parent);
    return createIndex(row, column, row);
}

QModelIndex ScriptsModel::parent(const QModelIndex &child) const
{
    Q_UNUSED(child);
    return QModelIndex();
}

int ScriptsModel::rowCount(const QModelIndex &parent) const
{
    if (parent.isValid()) return 0;
    else return entries.size();
}

int ScriptsModel::columnCount(const QModelIndex &parent) const
{
    Q_UNUSED(parent);
    return 2;
}

QVariant ScriptsModel::data(const QModelIndex &index, int role) const
{
    int idx = index.internalId();
    const Entry &entry = entries[idx];

    if (role == Qt::DisplayRole)
    {
        if (index.column() == 0) {
            return QVariant(entry.name);
        } else if (index.column() == 1) {
            quint64 size = wad->fileSize(entry.index);
            return QLocale().formattedDataSize(size);
        }
    }

    return QVariant();
}

QVariant ScriptsModel::headerData(int section, Qt::Orientation orientation, int role) const
{
    if (role == Qt::DisplayRole) {
        if (orientation == Qt::Horizontal) {
            if (section == 0) return tr("Name");
            else return tr("Size");
        }
    }

    return QVariant();
}
