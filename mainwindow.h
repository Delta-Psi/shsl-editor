#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QLabel>
#include <QMainWindow>
#include <QStandardItemModel>

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
    void on_actionOpen_Project_triggered();

private:
    Ui::MainWindow *ui;
    QLabel projectStatusLabel;

    QStandardItemModel wadListModel;
    QStandardItemModel wadFilesModel;
};
#endif // MAINWINDOW_H
