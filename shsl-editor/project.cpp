#include "error.h"
#include "project.h"

#include <QDir>

GameFiles::GameFiles(const QString &gameDir)
{
    QDir dir(gameDir);

    files[0] = files[1] = files[2] = files[3] = nullptr;
    try {
        files[DR2_DATA] = new Wad(dir.filePath("dr2_data.wad"));
        files[DR2_DATA_US] = new Wad(dir.filePath("dr2_data_us.wad"));
        files[DR2_DATA_KEYBOARD] = new Wad(dir.filePath("dr2_data_keyboard.wad"));
        files[DR2_DATA_KEYBOARD_US] = new Wad(dir.filePath("dr2_data_keyboard_us.wad"));
    }  catch (Error &e) {
        for (int i = 0; i < FILE_COUNT; ++i)
        {
            if (files[i]) delete files[i];
        }

        throw Error("Could not load game files", &e);
    }
}

GameFiles::~GameFiles()
{
    for (int i = 0; i < FILE_COUNT; ++i)
    {
        delete files[i];
    }
}
