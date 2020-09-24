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
    enum ImageFormat
    {
        IF_Unknown,
        IF_TGA,
        IF_PNG,
    };

    explicit ImageDetailView(QWidget *parent = nullptr);
    ~ImageDetailView();

    bool display(const QByteArray &data/*, ImageFormat format = IF_Unknown*/);

private:
    Ui::ImageDetailView *ui;

    QPixmap pixmap;
    QGraphicsScene scene;
};

#endif // IMAGEDETAILVIEW_H
