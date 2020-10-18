#ifndef SCRIPTSVIEW_H
#define SCRIPTSVIEW_H

#include <QWidget>

#include "project.h"
#include "wad.h"
#include "models/scripts.h"
#include "scriptsyntaxhighlighter.h"

namespace Ui {
class ScriptsView;
}

class ScriptsView : public QWidget
{
    Q_OBJECT

public:
    explicit ScriptsView(QWidget *parent = nullptr);
    ~ScriptsView();

    void setFiles(GameFiles *files);

private:
    Ui::ScriptsView *ui;

    ScriptsModel scriptsModel;
    ScriptSyntaxHighlighter *syntaxHighlighter;

public slots:
    void setEnabled(bool e);

private slots:
    void onScriptSelected(const QModelIndex &current, const QModelIndex &previous);
};

#endif // SCRIPTSVIEW_H
