HDT_JAVA=rdfhdt

.PHONY: all
all: build test

.PHONY: build
build: HdtJavaBenchmark.class

${HDT_JAVA}:
	wget -nc https://github.com/rdfhdt/hdt-java/releases/download/v3.0.9/rdfhdt.tar.gz
	tar xzf ${HDT_JAVA}.tar.gz
	#rm ${HDT_JAVA}.tar.gz
	#sed -i 's|INFO|OFF|' "${HDT_JAVA}/log4j2.properties"

HdtJavaBenchmark.class: ${HDT_JAVA} src/HdtJavaBenchmark.java
	javac -cp "`./cp.sh`" src/HdtJavaBenchmark.java -d .

.PHONY: clean
clean:
	rm -f HdtJavaBenchmark.class

.PHONY: clean-all
clean-all: clean
	rm -rf ${HDT_JAVA}

.PHONY: test
test: HdtJavaBenchmark.class
	./run query ../data/persondata_en_10k.ttl
