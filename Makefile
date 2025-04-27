main: main.rs
	rustc --edition 2021 -g -C link-args="-lc -lm" -C opt-level=z -C panic="abort" main.rs -o main
