INSTALLDIR = /usr/local/bin
MAIN = ddgui

dev:
	cargo build

run:
	cargo run

build:
	cargo build --release

install: build
	mkdir -p ${DESTDIR}/${INSTALLDIR}
	install -m 0755 target/release/ddgui ${DESTDIR}/${INSTALLDIR}/${MAIN}

uninstall:
	rm -f ${DESTDIR}/${INSTALLDIR}/${MAIN}

.PHONY: dev run build install
