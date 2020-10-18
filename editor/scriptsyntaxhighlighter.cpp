#include "scriptsyntaxhighlighter.h"

#include <QRegularExpression>

ScriptSyntaxHighlighter::ScriptSyntaxHighlighter(QTextDocument *parent):
    QSyntaxHighlighter(parent)
{
}

void ScriptSyntaxHighlighter::highlightBlock(const QString &text)
{
    QTextCharFormat opcodeFormat;
    opcodeFormat.setFontWeight(QFont::Bold);
    opcodeFormat.setForeground(Qt::darkCyan);

    QRegularExpression opcode("^([a-zA-Z_][a-zA-Z_0-9]*)($|[ \t])");
    auto i = opcode.globalMatch(text);
    while(i.hasNext())
    {
        auto match = i.next();
        setFormat(match.capturedStart(1), match.capturedLength(1), opcodeFormat);
    }

    QTextCharFormat numberFormat;
    numberFormat.setForeground(Qt::red);

    QRegularExpression number("(0x[0-9a-fA-F]+)|(0b[01]+)|([1-9][0-9]*)");
    i = number.globalMatch(text);
    while(i.hasNext())
    {
        auto match = i.next();
        setFormat(match.capturedStart(), match.capturedLength(), numberFormat);
    }
}
