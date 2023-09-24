PREFIX=/usr/local
INSTALL_DIR=$(PREFIX)/bin
TARGET=target/release

build:
	cargo build --release

install:
	install -m 0755 $(TARGET)/bmi $(INSTALL_DIR)  

clean:
	rm -rf $(INSTALL_DIR)/bmi
