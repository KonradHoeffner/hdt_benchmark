#include "../b_librdf/measures.h"
#include <HDTManager.hpp>
#include <iostream>
#include <regex>

using namespace std;
using namespace hdt;

#define RDF_TYPE "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
#define DBO_PERSON "http://dbpedia.org/ontology/Person"
#define DBR_VINCENT "http://dbpedia.org/resource/Vincent_Descombes_Sevoie"

int query(int argc, char *argv[]) {
  char *program=argv[0];
  char *task=argv[1];
  
 unsigned long long t0, t1;
  double time_load, time_first = 0, time_rest;
  long m1, mem_graph;
  // queries.
  t0 = get_nanosec();
  std::string s = argv[2];
  s = std::regex_replace(s, std::regex("ttl"), "hdt");
  const char* filename = s.c_str();
  fprintf(stderr, "%s: Parsing %s\n", program, filename);
	long m0 = get_vmsize();
  HDT *hdt = HDTManager::mapIndexedHDT(filename);
  t1 = get_nanosec();
  time_load = (t1-t0)/1e9;
  
	m1 = get_vmsize();
  //fprintf(stderr, "Memory before and after loading %i %i\n", m0, m1);
  mem_graph = m1-m0; // difference may be 0 with 10k triples
  
	t0 = get_nanosec();
   
  int count = 0;
  // Enumerate all triples matching a pattern ("" means any)
  IteratorTripleString *it = hdt->search("", RDF_TYPE, DBO_PERSON);
  while (it->hasNext()) {
    TripleString *triple = it->next();
    if (count == 0) {
      t1 = get_nanosec();
      time_first = (t1-t0)/1e9;
      t0 = get_nanosec();
    }
    count++;
    /*cout << triple->getSubject() <<
    ", " << triple->getPredicate() <<
    ", " << triple->getObject() << endl;*/
  }
  t1 = get_nanosec();
  time_rest = (t1-t0)/1e9;
  fprintf(stderr, "%s: got %d matching statements\n", program, count);
  printf("%f,%ld,%f,%f\n", time_load, mem_graph, time_first, time_rest);
  delete it;  // Remember to delete iterator to avoid memory leaks!
  delete hdt; // Remember to delete instance when no longer needed!
  return 0;
}

int main(int argc, char *argv[]) {
  if (argc < 3) {
    fprintf(stderr, "Usage: %s <task> <filename> [options...]\n", argv[0]);
    return 1;
  }
  const char *task = argv[1];
  /*if (strcmp(task, "parse") == 0) {
      return main_parse(argc, argv);
  } else */
  if (strcmp(task, "query") == 0) {
    return query(argc, argv);
  } /* else if (strcmp(task, "query2") == 0) {
       return main_query(argc, argv);
   } */
  else {
    fprintf(stderr, "Unknown task %s\n", task);
    return 1;
  }
}
