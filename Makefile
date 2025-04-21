main: main.o
	gcc -o main main.o -lm

main.o: main.rs
	rustc --edition 2021 -g -C opt-level=z --emit=obj -C panic="abort" main.rs

