#ifndef IMAGEDETAILVIEW_H
#define IMAGEDETAILVIEW_H

#include <QGraphicsScene>
#include <QStandardItemModel>
#include <QWidget>

namespace Ui {
class ImageDetailView;
}

class ImageDetailView : public QWidget
{
    Q_OBJECT

public:
    explicit ImageDetailView(QWidget *parent = nullptr);
    ~ImageDetailView();

    void display(const QByteArray &data);

private:
    Ui::ImageDetailView *ui;

    QPixmap pixmap;
    QGraphicsScene scene;
};

#endif // IMAGEDETAILVIEW_H
