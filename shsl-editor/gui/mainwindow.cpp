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
    wad = nullptr;

    ui->setupUi(this);

    // set up status bar
    statusBar()->addWidget(&projectStatusLabel);
    projectStatusLabel.setText("No project loaded");

    // set up models
    ui->wadFileTree->setModel(&wadFilesModel);
    connect(ui->wadFileTree->selectionModel(), &QItemSelectionModel::currentChanged,
            this, &MainWindow::on_wadFileSelected);

    // set up hex view
    hexEdit = new QHexEdit;
    hexEdit->setBytesPerLine(16);
    hexEdit->setReadOnly(true);
    ui->wadFileHexTab->layout()->addWidget(hexEdit);

    // set up image view
    imageView = new ImageDetailView;
    ui->wadFileImageTab->layout()->addWidget(imageView);
}

MainWindow::~MainWindow()
{
    delete ui;
    if (wad) delete wad;
}

void MainWindow::on_actionSet_Game_Directory_triggered()
{
    QString path = QFileDialog::getExistingDirectory(this, tr("Set Game Directory"));
    if (path == "") return;
    QDir dir(path);

    if (wad) delete wad;
    try {
        wad = new Wad(dir.filePath("dr2_data.wad"));
    } catch(Error &e) {
        delete wad;
        wad = nullptr;

        Error("Could not load WAD file", &e).showAsMessageBox(this);
        return;
    }

    ui->wadFileTree->setEnabled(true);
    wadFilesModel.setWad(wad);
    ui->wadFileTree->header()->setSectionResizeMode(0, QHeaderView::Stretch);

    ui->wadList->setEnabled(true);
    ui->wadList->clear();
    ui->wadList->addItem("dr2_data.wad");
    ui->wadList->setCurrentRow(0);
}

void MainWindow::on_wadFileSelected(const QModelIndex &current, const QModelIndex &previous)
{
    Q_UNUSED(previous);
    if (!wad) return;
    if (!wadFilesModel.canReadEntry(current)) return;

    QByteArray data = wadFilesModel.readEntry(current);

    ui->wadFileTabs->setEnabled(true);

    hexEdit->setEnabled(true);
    hexEdit->setData(data);
    hexEdit->setAddressArea(true);

    try {
        imageView->display(data);
    } catch (Error &e) {
        statusBar()->showMessage(e.fullMessage());
    }
}
