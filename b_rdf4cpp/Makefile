# requires rdf4cpp to be installed on the system, see https://github.com/rdf4cpp/rdf4cpp

.PHONY: test
test: all
	./run query ../data/persondata_en_10k.ttl ntriples

.PHONY: all
all: run

# copied over from b_hdt_cpp, adapt to rdf4cpp
run: run.cpp /usr/local/lib/librdf4cpp.a
	libtool --mode=link g++ -O3 -march=native run.cpp -o run

.PHONY: clean
clean:
	rm -f run
