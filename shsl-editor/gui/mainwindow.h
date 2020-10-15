#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QLabel>
#include <QMainWindow>
#include <QStandardItemModel>
#include <QSortFilterProxyModel>

#include "project.h"
#include "wad.h"
#include "gamefilesview.h"
#include "scriptsview.h"

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

private:
    Ui::MainWindow *ui;

    QLabel projectStatusLabel;
    GameFilesView *gameFilesView;
    ScriptsView *scriptsView;

    GameFiles *files;
};
#endif // MAINWINDOW_H
