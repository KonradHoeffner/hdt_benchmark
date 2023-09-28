# requires rdf4cpp to be installed on the system, see https://github.com/rdf4cpp/rdf4cpp

.PHONY: test
test: all
	./run query ../data/persondata_en_10k.ttl ntriples

.PHONY: all
all: run

.PHONY: b_rdf4cpp_conan
b_rdf4cpp_conan:
	bash conan.sh

.PHONY: b_rdf4cpp_configure
b_rdf4cpp_configure: b_rdf4cpp_conan
	cmake -DCMAKE_BUILD_TYPE=Release -B build -DCMAKE_C_FLAGS=" -march=native" -DCMAKE_CXX_FLAGS=" -march=native"

.PHONY: b_rdf4cpp_build
b_rdf4cpp_build: b_rdf4cpp_configure
	cmake --build build --parallel



run: b_rdf4cpp_build
	cp build/run run

.PHONY: clean
clean:
	rm -rf run build
