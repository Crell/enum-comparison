
all: c java

.PHONY: c java

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
