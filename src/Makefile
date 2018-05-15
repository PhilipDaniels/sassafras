linktest: linktest.c ../target/debug/libsassafras.a
	gcc -o linktest linktest.c -L../target/debug -lsassafras -lpthread -ldl

all: linktest

clean:
	rm ./linktest
