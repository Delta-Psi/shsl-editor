#include "gamefilesview.h"
#include "ui_gamefilesview.h"
#include "error.h"

GameFilesView::GameFilesView(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::GameFilesView)
{
    ui->setupUi(this);
    _files = nullptr;

    // set up models
    wadFilesFilter.setSourceModel(&wadFilesModel);
    wadFilesFilter.setRecursiveFilteringEnabled(true);
    ui->wadFileTree->setModel(&wadFilesFilter);
    connect(ui->wadFileTree->selectionModel(), &QItemSelectionModel::currentChanged,
            this, &GameFilesView::on_wadFileSelected);

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

GameFilesView::~GameFilesView()
{
    delete ui;
}

void GameFilesView::setFiles(GameFiles *files)
{
    if (_files) delete _files;
    _files = files;

    wadFilesModel.setFiles(files);
    ui->wadFileTree->header()->setSectionResizeMode(0, QHeaderView::Stretch);
}

void GameFilesView::setEnabled(bool e)
{
    ui->wadList->setEnabled(e);
    ui->wadFileTabs->setEnabled(e);
    ui->wadFileTree->setEnabled(e);
    ui->wadFileTreeFilter->setEnabled(e);
    ui->wadFileTreeFilterReset->setEnabled(e);
}

void GameFilesView::on_wadFileSelected(const QModelIndex &current, const QModelIndex &previous)
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
    } catch (Error&) {
        // just don't show it
    }
}

void GameFilesView::on_wadFileTreeFilter_textChanged(const QString &filter)
{
    wadFilesFilter.setFilterFixedString(filter);
}

void GameFilesView::on_wadFileTreeFilterReset_clicked()
{
    ui->wadFileTreeFilter->clear();
}

void GameFilesView::on_wadFileTree_customContextMenuRequested(const QPoint &pos)
{
    Q_UNUSED(pos);
    QModelIndex index = ui->wadFileTree->indexAt(pos);
    index = wadFilesFilter.mapToSource(index);
    wadFilesModel.onRightClick(index, this);
}
