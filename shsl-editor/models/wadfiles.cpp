#include "wadfiles.h"

#include <QApplication>
#include <QStyle>
#include <algorithm>
#include <QDebug>
#include <QMenu>
#include <QFileDialog>
#include <QSaveFile>

WadFilesModel::WadFilesModel(Wad *wad)
    : _wad(wad)
{
}

void WadFilesModel::setWad(Wad *wad)
{
    _wad = wad;
    updateEntries();
}

bool WadFilesModel::canReadEntry(const QModelIndex &index)
{
    if (!index.isValid()) return false;

    return entries[index.internalId()].file;
}

QByteArray WadFilesModel::readEntry(const QModelIndex &index)
{
    return readEntry(entries[index.internalId()]);
}

void WadFilesModel::onRightClick(const QModelIndex &index, QWidget *menuParent)
{
    if (!index.isValid()) return;

    quint64 id = index.internalId();
    const Entry &entry = entries[id];
    if (entry.directory) return;

    QMenu menu(menuParent);

    QAction saveAs(tr("Save As..."), menuParent);
    connect(&saveAs, &QAction::triggered,
            [=](bool) {
        QFileDialog dialog(menuParent);
        dialog.setFileMode(QFileDialog::AnyFile);
        dialog.setAcceptMode(QFileDialog::AcceptSave);
        if (dialog.exec())
        {
            QByteArray data = readEntry(entry);
            QSaveFile file(dialog.selectedFiles()[0]);
            if (file.open(QIODevice::WriteOnly))
            {
                file.write(data);
                file.commit();
            }
        }
    });
    menu.addAction(&saveAs);

    menu.exec(QCursor::pos());
}

void WadFilesModel::clear()
{
    beginResetModel();
    entries.clear();
    endResetModel();
}

QModelIndex WadFilesModel::index(int row, int column, const QModelIndex &parent) const
{
    if (!_wad) return QModelIndex();

    quint64 parentId = 0;
    if (parent.isValid()) parentId = parent.internalId();
    const Entry &entry = entries[parentId];

    if (row < 0 or row >= entry.children.size()) return QModelIndex();
    return createIndex(row, column, entry.children[row]);
}

QModelIndex WadFilesModel::parent(const QModelIndex &child) const
{
    if (!_wad) return QModelIndex();

    const Entry &entry = entries[child.internalId()];
    if (entry.parent <= 0) return QModelIndex();
    return createIndex(entries[entry.parent].row, 0, entry.parent);
}

int WadFilesModel::rowCount(const QModelIndex &parent) const
{
    if (!_wad) return 0;
    if (entries.isEmpty()) return 0;

    if (parent.isValid())
    {
        return entries[parent.internalId()].children.size();
    } else {
        return entries[0].children.size();
    }
}

int WadFilesModel::columnCount(const QModelIndex &parent) const
{
    Q_UNUSED(parent);
    if (!_wad) return 0;

    return 2;
}

QVariant WadFilesModel::data(const QModelIndex &index, int role) const
{
    if (!_wad) return QVariant();

    if (role == Qt::DisplayRole) {
        const Entry &entry = entries[index.internalId()];
        if (index.column() == 0)
        {
            return entry.name;
        } else if (index.column() == 1) {
            if (entry.file)
            {
                quint64 size = _wad->fileSize(entry.index);
                return QLocale().formattedDataSize(size);
            }
        }
    } else if (role == Qt::DecorationRole) {
        if (index.column() == 0)
        {
            const Entry &entry = entries[index.internalId()];

            QStyle *style = QApplication::style();
            if (entry.directory) {
                return style->standardIcon(QStyle::SP_DirIcon);
            } else if (entry.file) {
                return style->standardIcon(QStyle::SP_FileIcon);
            }
        }
    }

    return QVariant();
}

void WadFilesModel::updateEntries()
{
    beginResetModel();

    entries.clear();
    // create the root entry
    Entry root;
    root.row = -1;
    root.parent = -1;
    root.file = false;
    root.directory = true;
    entries.append(root);

    updateEntriesSub(0, "");

    endResetModel();
}

void WadFilesModel::updateEntriesSub(int parentId, const QString &parentPath)
{
    int dirIndex = _wad->dirIndex(parentPath);
    const QVector<Wad::Dir::Subfile> &subfiles = _wad->dirSubfiles(dirIndex);

    for(auto subfile = subfiles.begin(); subfile != subfiles.end(); ++subfile)
    {
        Entry entry;
        entry.parent = parentId;
        entry.name = subfile->name;

        QString path;
        if (parentPath.size() > 0) path += parentPath + '/';
        path += entry.name;

        if (subfile->isDirectory)
        {
            entry.directory = true;
            entry.file = false;

            int index = _wad->dirIndex(path);
            Q_ASSERT(index >= 0);
            entry.index = index;
        } else {
            entry.directory = false;
            entry.file = true;

            int index = _wad->fileIndex(path);
            Q_ASSERT(index >= 0);
            entry.index = index;

            //entry.offset = _wad->fileOffset(index);
            //entry.size = _wad->fileSize(index);
        }

        int id = entries.size();
        entries[parentId].children.append(id);
        entries.append(entry);

        if (subfile->isDirectory)
        {
            updateEntriesSub(id, path);
        }
    }
    std::sort(entries[parentId].children.begin(),
          entries[parentId].children.end(),
          [this](int a, int b)->bool{return entries[a].name < entries[b].name;});
    for(int i = 0; i < entries[parentId].children.size(); ++i)
    {
        entries[entries[parentId].children[i]].row = i;
    }
}

QByteArray WadFilesModel::readEntry(const WadFilesModel::Entry &entry)
{
    return _wad->readFile(entry.index);
}
