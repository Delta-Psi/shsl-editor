#include "scripts.h"

ScriptsModel::ScriptsModel():
    wad(nullptr)
{
}

void ScriptsModel::setFiles(GameFiles *files)
{
    wad = files->get(GameFiles::DR2_DATA_US);
    updateEntries();
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
    Q_UNUSED(parent);
    return entries.size();
}

int ScriptsModel::columnCount(const QModelIndex &parent) const
{
    Q_UNUSED(parent);
    return 1;
}

QVariant ScriptsModel::data(const QModelIndex &index, int role) const
{
    int idx = index.internalId();
    const Entry &entry = entries[idx];

    if (role == Qt::DisplayRole)
    {
        return QVariant(entry.name);
    }

    return QVariant();
}
