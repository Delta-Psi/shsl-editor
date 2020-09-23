#include "mainwindow.h"
#include "ui_mainwindow.h"
#include "wad.h"

#include <QDebug>
#include <QFileDialog>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);
}

MainWindow::~MainWindow()
{
    delete ui;
}


void MainWindow::on_actionOpen_game_directory_triggered()
{
    QString path = QFileDialog::getExistingDirectory(this, tr("Open game directory"));
    Wad wad(path + "/dr2_data.wad");

    for(auto it = wad.files().constBegin(); it != wad.files().constEnd(); ++it) {
        qDebug() << it.key();
    }
}
