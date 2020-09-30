#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QLabel>
#include <QMainWindow>
#include <QStandardItemModel>

#include <models/wadfiles.h>

#include "qhexedit.h"
#include "imagedetailview.h"
#include "wad.h"

QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private slots:
    void on_actionSet_Game_Directory_triggered();

    void on_wadFileSelected(const QModelIndex &current, const QModelIndex &previous);

    void on_wadFileTree_customContextMenuRequested(const QPoint &pos);

private:
    Ui::MainWindow *ui;

    QHexEdit *hexEdit;
    ImageDetailView *imageView;

    QLabel projectStatusLabel;

    Wad *wad;

    WadFilesModel wadFilesModel;
};
#endif // MAINWINDOW_H
