
all: c java python rust

.PHONY: c java python rust

clean: clean-c clean-java clean-rust

c: c/main.c
	cd c && \
	gcc main.c && \
	./a.out

clean-c:
	rm -f c/a.out

java:
	cd java && \
	javac enum.java && \
	java Main.class


clean-java:
	cd java && rm -f *.class

python:
	cd python && \
	python3 enumtest.py

rust:
	cd rust && \
	cargo run

clean-rust:
	cd rust && \
	cargo clean