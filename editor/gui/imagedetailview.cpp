#include "imagedetailview.h"
#include "ui_imagedetailview.h"
#include "helper.h"
#include "error.h"
#include <QtDebug>

ImageDetailView::ImageDetailView(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::ImageDetailView)
{
    ui->setupUi(this);

    ui->splitter->setStretchFactor(0, 2);

    ui->graphicsView->setScene(&scene);

    ui->paletteView->hide();
}

ImageDetailView::~ImageDetailView()
{
    delete ui;
}

void ImageDetailView::display(const QByteArray &data)
{
    scene.clear();
    ui->paletteView->hide();
    ui->details->clear();

    // try loading it as a TGA
    MemoryInterface intf((uint8_t*)data.data(), data.size());
    tga::Decoder decoder(&intf);
    tga::Header header;
    if (decoder.readHeader(header)) {
        // it SHOULD be one. fill out metadata
        ui->details->setColumnCount(1);
        ui->details->horizontalHeader()->setSectionResizeMode(0, QHeaderView::Stretch);
        ui->details->setRowCount(1);
        ui->details->setVerticalHeaderLabels({tr("Format")});

        ui->details->setItem(0, 0, new QTableWidgetItem(tr("TGA")));

        tga::Image tgaImage;
        tgaImage.bytesPerPixel = header.bytesPerPixel();
        tgaImage.rowstride = header.width * header.bytesPerPixel();
        size_t bufferSize = tgaImage.rowstride * header.height;
        uint8_t *buffer = (uint8_t*)malloc(bufferSize);
        tgaImage.pixels = buffer;
        if (!decoder.readImage(header, tgaImage, nullptr)) {
            throw Error("Unable to read TGA pixel data");
        }
        QImage::Format format = QImage::Format_Grayscale8;
        if (header.hasColormap())
        {
            format = QImage::Format_Indexed8;
        } else if (header.isRgb()) {
            if (header.bytesPerPixel() != 4) throw Error("Unknown image type: rgb, bpp not 4");
            // BGRA -> RGBA
            for (size_t i = 0; i < bufferSize; i += 4)
            {
                std::swap(buffer[i], buffer[i+2]);
            }
            format = QImage::Format_ARGB32_Premultiplied;
        } else {
            if (header.bytesPerPixel() != 1) throw Error("Unknown image type: gray, bpp not 1");
            format = QImage::Format_Grayscale8;
        }

        QImage image(buffer, header.width, header.height, format,
                 [](void *buffer) {free(buffer);}, buffer);

        if (header.hasColormap())
        {
            ui->paletteView->show();
            ui->paletteView->clearContents();
            image.setColorCount(header.colormapLength);
            for(int i = 0; i < header.colormapLength; ++i)
            {
                QRgb color = qRgba(tga::getr(header.colormap[i]),
                           tga::getg(header.colormap[i]),
                           tga::getb(header.colormap[i]),
                           tga::geta(header.colormap[i]));
                image.setColor(i, color);

                QTableWidgetItem *item = new QTableWidgetItem("");
                item->setBackground(QBrush(color));
                ui->paletteView->setItem(i/16, i%16, item);
            }
        }

        if (!pixmap.convertFromImage(image))
        {
            throw Error("Could not convert image to pixmap");
        }
    } else {
        // not a tga! try the generic qt thingie
        if (!pixmap.loadFromData((const uchar*)data.data(), data.size()))
        {
            throw Error("Unknown image format");
        }
    }

    scene.addPixmap(pixmap);
    ui->graphicsView->setSceneRect(0, 0, pixmap.width(), pixmap.height());
}
