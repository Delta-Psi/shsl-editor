cmake_minimum_required(VERSION 3.7.0)
project("SHSL Editor" VERSION 0.1)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)
find_package(Qt5 COMPONENTS Widgets REQUIRED)

# qhexhedit2
add_library(qhexedit2
	qhexedit2/src/qhexedit.cpp
	qhexedit2/src/chunks.cpp
	qhexedit2/src/commands.cpp
	)
target_compile_definitions(qhexedit2 PUBLIC QHEXEDIT_EXPORTS)
target_link_libraries(qhexedit2 Qt5::Widgets)

# tga
add_library(tga
	tga/decoder.cpp
	tga/encoder.cpp
	tga/image_iterator.cpp
)

include(FetchContent)
FetchContent_Declare(
	Corrosion
	GIT_REPOSITORY https://github.com/AndrewGaspar/corrosion.git
	GIT_TAG origin/master
	)
FetchContent_MakeAvailable(Corrosion)
set(ENV{BINDINGS_OUT_DIR} "fuck")
corrosion_import_crate(MANIFEST_PATH shsl-library/Cargo.toml)

add_subdirectory(shsl-editor)
