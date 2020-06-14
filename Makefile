
all: c java

c: c/a.out
	cd c&& gcc main.c


java:
	cd java && javac enum.java

