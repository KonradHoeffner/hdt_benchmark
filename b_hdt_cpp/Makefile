# requires libtool and hdt-cpp to be installed on the system, see https://github.com/rdfhdt/hdt-cpp

.PHONY: test
test: all
	./run query ../data/persondata_en_10k.ttl ntriples

.PHONY: all
all: run

run: run.cpp /usr/local/lib/libhdt.la ../b_librdf/measures.c
	libtool --mode=link g++ -O3 -march=native /usr/local/lib/libhdt.la ../b_librdf/measures.c run.cpp -o run

.PHONY: clean
clean:
	rm -f run
