#include "imagedetailview.h"
#include "ui_imagedetailview.h"

ImageDetailView::ImageDetailView(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::ImageDetailView)
{
    ui->setupUi(this);

    ui->graphicsView->setScene(&scene);
}

ImageDetailView::~ImageDetailView()
{
    delete ui;
}

bool ImageDetailView::display(const QByteArray &data/*, ImageDetailView::ImageFormat format*/)
{
    scene.clear();

    if (pixmap.loadFromData((const uchar*)data.data(), data.size()))
    {
        // probably a PNG
    }
    else
    {
        // try loading it as a TGA

        //QImage image(buffer, x, y, format);

        /*if (!pixmap.convertFromImage(image))
        {
            return false;
        }*/
        return false;
    }

    scene.addPixmap(pixmap);
    return true;
}
