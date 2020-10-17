#ifndef ERROR_H
#define ERROR_H

#include <QException>
#include <QMessageBox>

class Error : public QException
{
public:
    Error(const QString& message, Error *cause=nullptr):
        _message(message), _cause(cause) { if (_cause) _cause = _cause->clone(); }
    ~Error() { if (_cause) delete _cause; }

    Error *cause() const { return _cause; };
    const QString &message() const { return _message; }
    QString fullMessage() const {
        QString result = _message;
        if (_cause) result += ": " + _cause->fullMessage();
        return result;
    }

    void showAsMessageBox(QWidget *parent) const
    {
        QMessageBox msg(QMessageBox::Warning, "Error", fullMessage(), QMessageBox::NoButton, parent);
        msg.exec();
    }

    void raise() const override { throw *this; }
    Error *clone() const override { return new Error(_message); }

private:
    QString _message;
    Error *_cause;
};

#endif // ERROR_H
