#include "scriptsyntaxhighlighter.h"

#include <QRegularExpression>
#include <QDebug>

ScriptSyntaxHighlighter::ScriptSyntaxHighlighter(QTextDocument *parent):
        QSyntaxHighlighter(parent)
{
}

enum State {
    DEFAULT = -1,
    STRING = 0,
};

void ScriptSyntaxHighlighter::highlightBlock(const QString &text)
{
    QRegularExpression stringStart("`$");
    QRegularExpression stringEnd("(^|[^\\\\])`");

    setCurrentBlockState(previousBlockState());
    int currentPosition = 0;
    while (currentPosition < text.size())
    {
        if (currentBlockState() == DEFAULT)
        {
            // see if a string begins
            auto stringMatch = stringStart.match(text, currentPosition);
            if (stringMatch.hasMatch())
            {
                setCurrentBlockState(STRING);
                highlightDefault(text, currentPosition, stringMatch.capturedStart());
                currentPosition = stringMatch.capturedEnd();
            } else {
                highlightDefault(text, currentPosition, text.size());
                break;
            }
        } else if (currentBlockState() == STRING) {
            // see if the string ends
            auto stringEndMatch = stringEnd.match(text);
            if (stringEndMatch.hasMatch())
            {
                setCurrentBlockState(DEFAULT);
                highlightString(text, currentPosition, stringEndMatch.capturedEnd());
                currentPosition = stringEndMatch.capturedEnd();
            } else {
                highlightString(text, currentPosition, text.size());
                break;
            }
        }
    }
}

void ScriptSyntaxHighlighter::highlightDefault(const QString &text, int begin, int end)
{
    QRegularExpression opcode("^([a-zA-Z_][a-zA-Z_0-9]*)($|[ \t])");
    QRegularExpression number("(^|[ \t,])(0x[0-9a-fA-F]+)|(0b[01]+)|([1-9][0-9]*)($|[ \t,])");

    // match opcodes
    auto i = opcode.globalMatch(text, begin);
    while(i.hasNext())
    {
        auto match = i.next();
        if (match.capturedEnd() > end) break;
        setFormat(match.capturedStart(1), match.capturedLength(1), Qt::darkCyan);
    }

    // match numbers
    i = number.globalMatch(text, begin);
    while(i.hasNext())
    {
        auto match = i.next();
        if (match.capturedEnd() > end) break;
        setFormat(match.capturedStart(), match.capturedLength(), Qt::red);
    }
}

void ScriptSyntaxHighlighter::highlightString(const QString &text, int offset, int end)
{
    setFormat(offset, end - offset, Qt::gray);
}
