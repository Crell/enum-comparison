
all: c java

.PHONY: c java

clean: clean-c clean-java

c: c/main.c
	cd c && gcc main.c

clean-c:
	rm -f c/a.out

java:
	cd java && javac enum.java

clean-java:
	cd java && rm -f *.class
