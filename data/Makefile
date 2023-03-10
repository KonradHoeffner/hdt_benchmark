.PHONY: all
all: persondata_en.ttl persondata_en_10k.ttl persondata_en_20k.ttl persondata_en_40k.ttl persondata_en_80k.ttl persondata_en_100k.ttl persondata_en_1M.ttl persondata_en_10k.hdt persondata_en_20k.hdt persondata_en_40k.hdt persondata_en_80k.hdt persondata_en_100k.hdt persondata_en_1M.hdt persondata_en.hdt

persondata_en.ttl:
	wget http://downloads.dbpedia.org/2016-10/core-i18n/en/persondata_en.ttl.bz2
	bunzip2 persondata_en.ttl.bz2

persondata_en_10k.ttl: persondata_en.ttl
	tail -10000 "$<" >"$@"

persondata_en_20k.ttl: persondata_en.ttl
	tail -20000 "$<" >"$@"

persondata_en_30k.ttl: persondata_en.ttl
	tail -30000 "$<" >"$@"

persondata_en_40k.ttl: persondata_en.ttl
	tail -40000 "$<" >"$@"

persondata_en_80k.ttl: persondata_en.ttl
	tail -80000 "$<" >"$@"

persondata_en_100k.ttl: persondata_en.ttl
	tail -100000 "$<" >"$@"

persondata_en_1M.ttl: persondata_en.ttl
	tail -1000000 "$<" >"$@"

persondata_en_10k.hdt: persondata_en_10k.ttl
	docker run -v ${PWD}:/data rdfhdt/hdt-cpp rdf2hdt /data/persondata_en_10k.ttl /data/persondata_en_10k.hdt

persondata_en_20k.hdt: persondata_en_20k.ttl
	docker run -v ${PWD}:/data rdfhdt/hdt-cpp rdf2hdt /data/persondata_en_20k.ttl /data/persondata_en_20k.hdt

persondata_en_30k.hdt: persondata_en_30k.ttl
	docker run -v ${PWD}:/data rdfhdt/hdt-cpp rdf2hdt /data/persondata_en_30k.ttl /data/persondata_en_30k.hdt

persondata_en_40k.hdt: persondata_en_40k.ttl
	docker run -v ${PWD}:/data rdfhdt/hdt-cpp rdf2hdt /data/persondata_en_40k.ttl /data/persondata_en_40k.hdt

persondata_en_80k.hdt: persondata_en_80k.ttl
	docker run -v ${PWD}:/data rdfhdt/hdt-cpp rdf2hdt /data/persondata_en_80k.ttl /data/persondata_en_80k.hdt

persondata_en_100k.hdt: persondata_en_100k.ttl
	docker run -v ${PWD}:/data rdfhdt/hdt-cpp rdf2hdt /data/persondata_en_100k.ttl /data/persondata_en_100k.hdt

persondata_en_1M.hdt: persondata_en_1M.ttl
	docker run -v ${PWD}:/data rdfhdt/hdt-cpp rdf2hdt /data/persondata_en_1M.ttl /data/persondata_en_1M.hdt

persondata_en.hdt: persondata_en.ttl
	docker run -v ${PWD}:/data rdfhdt/hdt-cpp rdf2hdt /data/persondata_en.ttl /data/persondata_en.hdt
