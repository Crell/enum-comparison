
all: c java python

.PHONY: c java python

clean: clean-c clean-java

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
