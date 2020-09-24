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
    ui->wadList->setModel(&wadListModel);
    ui->wadFileTree->setModel(&wadFilesModel);

    // set up hex view
    hexEdit = new QHexEdit;
    hexEdit->setBytesPerLine(16);
    hexEdit->setReadOnly(true);
    ui->wadFileHexTab->layout()->addWidget(hexEdit);
}

MainWindow::~MainWindow()
{
    delete ui;
    if (wad) delete wad;
}

void setupWadFilesModel(QStandardItemModel& model, const Wad* wad)
{
    model.clear();

    QHash<QStringRef, QStandardItem*> paths;

    QStandardItem* root = model.invisibleRootItem();
    for(auto it = wad->files().constBegin(); it != wad->files().constEnd(); ++it)
    {
        const QString& path = it.key();
        QVector<QStringRef> split = path.splitRef('/');

        // ensure elements for each parent directory exist
        QStandardItem* parent = root;
        for(int i = 0; i < split.size(); ++i)
        {
            QStringRef currPath = path.leftRef(split[i].position() + split[i].size());

            if (paths.contains(currPath))
            {
                parent = paths[currPath];
            } else {
                QStandardItem* current = new QStandardItem(QString(split[i].data(), split[i].size()));
                current->setData(QVariant(QString(currPath.data(), currPath.size())));
                paths.insert(currPath, current);

                parent->appendRow(current);
                parent = current;
            }
        }
    }

    model.sort(0);
}

void MainWindow::on_actionSet_Game_Directory_triggered()
{
    QDir path(QFileDialog::getExistingDirectory(this, tr("Set Game Directory")));
    if (wad) delete wad;
    wad = new Wad(path.filePath("dr2_data.wad"));
    if (!wad->open())
    {
        statusBar()->showMessage("could not open game files");
        delete wad;
        wad = nullptr;
    } else {
        ui->wadFileTree->setEnabled(true);
        setupWadFilesModel(wadFilesModel, wad);

        ui->wadList->setEnabled(true);
        wadListModel.clear();
        wadListModel.appendRow(new QStandardItem("dr2_data.wad"));
        ui->wadList->setCurrentIndex(wadListModel.index(0, 0));
    }
}

void MainWindow::on_wadFileTree_clicked(const QModelIndex &index)
{
    if (!wad) return;

    QStandardItem *item = wadFilesModel.itemFromIndex(index);
    Q_ASSERT(item);
    QString path = item->data().toString();

    if (!wad->containsFile(path)) return;
    QByteArray data = wad->readFile(path);

    ui->wadFileTabs->setEnabled(true);

    hexEdit->setEnabled(true);
    hexEdit->setData(data);
}
