.PHONY: clear run watch

clear:
	rm ~/.emu.toml

run:
	cargo run

watch:
	watch -n 1 bat ~/.emu.toml
