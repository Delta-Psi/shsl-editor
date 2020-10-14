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
    ui->wadList->hide(); // until i figure out what to even do with this
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

    ui->wadFileTree->setEnabled(true);
    wadFilesModel.setFiles(files);
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
