#ifndef SCRIPT_H
#define SCRIPT_H

#include <QByteArray>
#include <QString>
#include <QVector>

class Script
{
public:
    static Script decode(const QByteArray &data);

private:
    QVector<QString> strings;
};

#endif // SCRIPT_H
