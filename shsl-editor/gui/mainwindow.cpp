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
    dr2_data = dr2_data_us = nullptr;

    ui->setupUi(this);

    // set up status bar
    statusBar()->addWidget(&projectStatusLabel);
    projectStatusLabel.setText("No project loaded");

    // set up models
    wadFilesFilter.setSourceModel(&wadFilesModel);
    wadFilesFilter.setRecursiveFilteringEnabled(true);
    ui->wadFileTree->setModel(&wadFilesFilter);
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

    ui->wadFileTreeFilterReset->setIcon(style()->standardIcon(QStyle::SP_DialogResetButton));
    ui->wadList->horizontalHeader()->setSectionResizeMode(0, QHeaderView::Stretch);
}

MainWindow::~MainWindow()
{
    delete ui;
    if (dr2_data) delete dr2_data;
    if (dr2_data_us) delete dr2_data_us;
}

void MainWindow::on_actionSet_Game_Directory_triggered()
{
    QString path = QFileDialog::getExistingDirectory(this, tr("Set Game Directory"));
    if (path == "") return;
    QDir dir(path);

    if (dr2_data) delete dr2_data;
    if (dr2_data_us) delete dr2_data_us;
    try {
        dr2_data = new Wad(dir.filePath("dr2_data.wad"));
        dr2_data_us = new Wad(dir.filePath("dr2_data_us.wad"));
    } catch(Error &e) {
        if (dr2_data) delete dr2_data;
        if (dr2_data_us) delete dr2_data_us;
        dr2_data = dr2_data_us = nullptr;

        Error("Could not load WAD files", &e).showAsMessageBox(this);
        return;
    }

    ui->wadFileTree->setEnabled(true);
    wadFilesModel.setWads(dr2_data, dr2_data_us);
    ui->wadFileTree->header()->setSectionResizeMode(0, QHeaderView::Stretch);

    ui->wadList->setEnabled(true);

    ui->wadFileTreeFilter->setEnabled(true);
    ui->wadFileTreeFilterReset->setEnabled(true);
}

void MainWindow::on_wadFileSelected(const QModelIndex &current, const QModelIndex &previous)
{
    Q_UNUSED(previous);
    if (!current.isValid()) return;

    QModelIndex index = wadFilesFilter.mapToSource(current);
    if (!wadFilesModel.canReadEntry(index)) return;

    QByteArray data = wadFilesModel.readEntry(index);

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

void MainWindow::on_wadFileTree_customContextMenuRequested(const QPoint &pos)
{
    Q_UNUSED(pos);
    QModelIndex index = ui->wadFileTree->indexAt(pos);
    index = wadFilesFilter.mapToSource(index);
    wadFilesModel.onRightClick(index, this);
}

void MainWindow::on_wadFileTreeFilter_textChanged(const QString &filter)
{
    wadFilesFilter.setFilterFixedString(filter);
}

void MainWindow::on_wadFileTreeFilterReset_clicked()
{
    ui->wadFileTreeFilter->clear();
}
