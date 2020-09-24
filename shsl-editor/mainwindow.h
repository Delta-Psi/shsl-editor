#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QLabel>
#include <QMainWindow>
#include <QStandardItemModel>

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

    void on_wadFileTree_clicked(const QModelIndex &index);

private:
    Ui::MainWindow *ui;

    QHexEdit *hexEdit;
    ImageDetailView *imageView;

    QLabel projectStatusLabel;

    Wad *wad;

    QStandardItemModel wadFilesModel;
};
#endif // MAINWINDOW_H
