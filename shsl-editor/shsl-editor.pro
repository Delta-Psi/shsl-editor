QT       += core gui

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

CONFIG += c++11

# You can make your code fail to compile if it uses deprecated APIs.
# In order to do so, uncomment the following line.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

SOURCES += \
    ../tga/decoder.cpp \
    ../tga/encoder.cpp \
    ../tga/image_iterator.cpp \
    gui/imagedetailview.cpp \
    gui/mainwindow.cpp \
    helper.cpp \
    main.cpp \
    models/wadfiles.cpp \
    project.cpp \
    script/script.cpp \
    wad.cpp

HEADERS += \
    ../tga/tga.h \
    error.h \
    gui/imagedetailview.h \
    gui/mainwindow.h \
    helper.h \
    models/wadfiles.h \
    project.h \
    script/script.h \
    wad.h

FORMS += \
    gui/imagedetailview.ui \
    gui/mainwindow.ui

OTHER_FILES += checksums.json

# Default rules for deployment.
qnx: target.path = /tmp/$${TARGET}/bin
else: unix:!android: target.path = /opt/$${TARGET}/bin
!isEmpty(target.path): INSTALLS += target

win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../qhexedit2/src/release/ -lqhexedit
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../qhexedit2/src/debug/ -lqhexedit
else:unix: LIBS += -L$$OUT_PWD/../qhexedit2/src/ -lqhexedit

INCLUDEPATH += $$PWD/../qhexedit2/src
DEPENDPATH += $$PWD/../qhexedit2/src
