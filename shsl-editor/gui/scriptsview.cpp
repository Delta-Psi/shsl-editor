#include "scriptsview.h"
#include "ui_scriptsview.h"
#include "script/script.h"

#include <QDebug>


ScriptsView::ScriptsView(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::ScriptsView)
{
    ui->setupUi(this);

    ui->scriptList->setModel(&scriptsModel);
    connect(ui->scriptList->selectionModel(), &QItemSelectionModel::currentChanged,
            this, &ScriptsView::onScriptSelected);
    ui->scriptList->header()->setSectionResizeMode(0, QHeaderView::Stretch);

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

void ScriptsView::onScriptSelected(const QModelIndex &current, const QModelIndex &previous)
{
    ui->scriptEdit->clear();

    Q_UNUSED(previous);
    QByteArray scriptData = scriptsModel.readEntry(current);
    Script script;
    if (!script.decode(scriptData)) {
        return;
    }

    for(const QString &string: script.getStrings()) {
        ui->scriptEdit->appendPlainText(string);
        ui->scriptEdit->appendPlainText("\n");
    }
}
