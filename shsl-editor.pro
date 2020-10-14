TEMPLATE = subdirs

SUBDIRS = \
	shsl-editor \
	qhexedit2

shsl-editor.subdir = shsl-editor
qhexedit2.file = qhexedit2/src/qhexedit.pro

shsl-editor.depends = qhexedit2
