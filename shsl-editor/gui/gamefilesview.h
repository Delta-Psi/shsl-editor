#ifndef GAMEFILESVIEW_H
#define GAMEFILESVIEW_H

#include <QSortFilterProxyModel>
#include <QWidget>

#include "project.h"
#include "models/wadfiles.h"
#include "qhexedit.h"
#include "imagedetailview.h"

namespace Ui {
class GameFilesView;
}

class GameFilesView : public QWidget
{
    Q_OBJECT

public:
    explicit GameFilesView(QWidget *parent = nullptr);
    ~GameFilesView();

    void setFiles(GameFiles *files);

private:
    Ui::GameFilesView *ui;

    QHexEdit *hexEdit;
    ImageDetailView *imageView;

    GameFiles *_files;

    QSortFilterProxyModel wadFilesFilter;
    WadFilesModel wadFilesModel;

public slots:
    void setEnabled(bool e);

private slots:
    void on_wadFileSelected(const QModelIndex &current, const QModelIndex &previous);
    void on_wadFileTreeFilter_textChanged(const QString &filter);
    void on_wadFileTreeFilterReset_clicked();
    void on_wadFileTree_customContextMenuRequested(const QPoint &pos);
};

#endif // GAMEFILESVIEW_H
