#include "error.h"
#include "mainwindow.h"
#include "ui_mainwindow.h"
#include "wad.h"

#include <QDebug>
#include <QFileDialog>
#include <QLabel>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    files = nullptr;

    ui->setupUi(this);

    // set up status bar
    statusBar()->addWidget(&projectStatusLabel);
    projectStatusLabel.setText("No project loaded");

    gameFilesView = new GameFilesView;
    ui->gameFilesTab->layout()->addWidget(gameFilesView);

    scriptsView = new ScriptsView;
    ui->scriptsTab->layout()->addWidget(scriptsView);
}

MainWindow::~MainWindow()
{
    delete ui;
    if (files) delete files;
}

void MainWindow::on_actionSet_Game_Directory_triggered()
{
    QString path = QFileDialog::getExistingDirectory(this, tr("Set Game Directory"));
    if (path == "") return;
    QDir dir(path);

    if (files) delete files;
    try {
        files = new GameFiles(path);
    } catch(Error &e) {
        files = nullptr;
        e.showAsMessageBox(this);
        return;
    }

    gameFilesView->setEnabled(true);
    gameFilesView->setFiles(files);
    scriptsView->setEnabled(true);
    scriptsView->setFiles(files);
}
