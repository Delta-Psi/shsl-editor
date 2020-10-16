add_executable(shsl-editor
	gui/gamefilesview.cpp
	gui/imagedetailview.cpp
	gui/mainwindow.cpp
	gui/scriptsview.cpp
	helper.cpp
	main.cpp
	models/scripts.cpp
	models/wadfiles.cpp
	project.cpp
	script/script.cpp
	wad.cpp

	gui/gamefilesview.ui
	gui/imagedetailview.ui
	gui/mainwindow.ui
	gui/scriptsview.ui
	)
target_include_directories(shsl-editor PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(shsl-editor Qt5::Widgets)

target_link_libraries(shsl-editor qhexedit2)
target_include_directories(shsl-editor PRIVATE "${PROJECT_SOURCE_DIR}/qhexedit2/src")

target_link_libraries(shsl-editor tga)
