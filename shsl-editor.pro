TEMPLATE = subdirs

SUBDIRS = \
	shsl-editor \
        qhexedit2

qhexedit2.file = qhexedit2/src/qhexedit.pro

shsl-editor.depends = qhexedit2
