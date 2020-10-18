#ifndef SHSL_LIBRARY_H
#define SHSL_LIBRARY_H

namespace ffi {
	struct Script;
}

#include "shsl-library_ffi.h"

#include <stdexcept>
#include <QtCore>

class Script {
public:
	Script() = delete;
	Script(const Script&) = delete;
	Script(Script&&) = default;

	static Script decode(const QByteArray &data) {
		ffi::Script *inner = ffi::decode_script(data.data(), data.size());
		if (!inner) throw std::runtime_error("could not decode script");
		return Script(inner);
	}

	size_t stringCount() {
		return ffi::script_string_count(inner);
	}
	QString getString(size_t index) {
		ffi::Data data = ffi::script_string_get(inner, index);
		return QString::fromUtf8(data.ptr, data.size);
	}
	
	~Script() {
		delete_script(inner);
	}

private:
	Script(ffi::Script *inner): inner(inner) {}

	ffi::Script *inner;
};

#endif
