#include "imagedetailview.h"
#include "ui_imagedetailview.h"
#include "helper.h"
#include <QtDebug>

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
        MemoryInterface intf((uint8_t*)data.data(), data.size());
        tga::Decoder decoder(&intf);
        tga::Header header;
        if (!decoder.readHeader(header)) return false;

        tga::Image tgaImage;
        tgaImage.bytesPerPixel = header.bytesPerPixel();
        tgaImage.rowstride = header.width * header.bytesPerPixel();
        uint8_t *buffer = (uint8_t*)malloc(tgaImage.rowstride * header.height);
        tgaImage.pixels = buffer;
        if (!decoder.readImage(header, tgaImage, nullptr))
        {
            free(buffer);
            return false;
        }

        QImage::Format format = QImage::Format_Grayscale8;
        if (header.hasColormap())
        {
            format = QImage::Format_Indexed8;
        } else {
            qDebug() << "unimplemented: non-indexed tga";
            return false;
        }

        QImage image(buffer, header.width, header.height, format,
            [](void *buffer) {free(buffer);}, buffer);

        if (header.hasColormap())
        {
            image.setColorCount(header.colormapLength);
            for(int i = 0; i < header.colormapLength; ++i)
            {
                image.setColor(i, qRgba(
                                   tga::getr(header.colormap[i]),
                                   tga::getg(header.colormap[i]),
                                   tga::getb(header.colormap[i]),
                                   tga::geta(header.colormap[i])));
            }
        }

        if (!pixmap.convertFromImage(image))
        {
            return false;
        }
    }

    scene.addPixmap(pixmap);
    return true;
}
