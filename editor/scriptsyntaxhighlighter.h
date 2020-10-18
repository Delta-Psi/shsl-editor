#ifndef SCRIPTSYNTAXHIGHLIGHTER_H
#define SCRIPTSYNTAXHIGHLIGHTER_H

#include <QSyntaxHighlighter>

class ScriptSyntaxHighlighter : public QSyntaxHighlighter
{
public:
    ScriptSyntaxHighlighter(QTextDocument *parent);

    // QSyntaxHighlighter interface
protected:
    void highlightBlock(const QString &text) override;
};

#endif // SCRIPTSYNTAXHIGHLIGHTER_H
