#ifndef SCRIPT_H
#define SCRIPT_H

#include <QByteArray>
#include <QString>
#include <QVector>

class Script
{
public:
    bool decode(const QByteArray &data);

    const QVector<QString> &getStrings() { return strings; }

private:
    QVector<QString> strings;

    bool readInstructions(const QByteArray &data);
    bool readStrings(const QByteArray &data);
};

#endif // SCRIPT_H
