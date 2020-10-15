#include "scriptsview.h"
#include "ui_scriptsview.h"

ScriptsView::ScriptsView(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::ScriptsView)
{
    ui->setupUi(this);

    ui->scriptList->setModel(&scriptsModel);
    ui->scriptEdit->document()->setDefaultFont(QFont("monospace"));
}

ScriptsView::~ScriptsView()
{
    delete ui;
}

void ScriptsView::setFiles(GameFiles *files)
{
    scriptsModel.setFiles(files);
}

void ScriptsView::setEnabled(bool e)
{
    ui->scriptList->setEnabled(e);
    ui->scriptEdit->setEnabled(e);
}
