all:build install run

build:
	cargo build --release
run:
	wallorganizer ./example

install:build
	cp ./target/release/wallorganizer /usr/local/bin/
	echo 'export PATH="$$PATH:/usr/local/bin"' >> ~/.bashrc
	echo 'export PATH="$$PATH:/usr/local/bin"' >> ~/.zshrc
	source ~/.bashrc || source ~/.zshrc || true
	@echo "Installation complete. If wallorganizer is not available, restart your terminal or run 'source ~/.bashrc' or 'source ~/.zshrc'"

uninstall:
	rm -f /usr/local/bin/wallorganizer
	sed -i '/export PATH=\"\$\$PATH:\/usr\/local\/bin\"/d' ~/.bashrc
	sed -i '/export PATH=\"\$\$PATH:\/usr\/local\/bin\"/d' ~/.zshrc
	@echo "Uninstallation complete. Please restart your terminal or run 'source ~/.bashrc' or 'source ~/.zshrc'"
