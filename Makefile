ARGS="-lc -lm"

main: main.rs
	rustc --edition 2021 -g -C link-args=$(ARGS) -C opt-level=z -C panic="abort" main.rs -o main

