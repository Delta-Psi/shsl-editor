#ifndef PROJECT_H
#define PROJECT_H

#include <QString>
#include "wad.h"

class GameFiles
{
public:
    enum File
    {
        DR2_DATA=0,
        DR2_DATA_US,
        DR2_DATA_KEYBOARD,
        DR2_DATA_KEYBOARD_US,
        FILE_COUNT,
    };

    GameFiles(const QString &gameDir);
    ~GameFiles();

    Wad *get(File index);

private:
    Wad *files[FILE_COUNT];
};

/*
class Project
{
public:
    Project(const QString &path);

private:
};
*/

#endif // PROJECT_H
