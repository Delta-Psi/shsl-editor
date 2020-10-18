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

private:
    void highlightDefault(const QString &text, int begin, int end);
    void highlightString(const QString &text, int begin, int end);
};

#endif // SCRIPTSYNTAXHIGHLIGHTER_H
