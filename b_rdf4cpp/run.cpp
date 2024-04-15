#include <rdf4cpp.hpp>
#include <measures.h>

#include <iostream>

using namespace std;

#define RDF_TYPE "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
#define DBO_PERSON "http://dbpedia.org/ontology/Person"
#define DBR_VINCENT "http://dbpedia.org/resource/Vincent_Descombes_Sevoie"

enum class benchmark_queries : u_int8_t {
    query_1 = 1,
    query_2 = 2
};

void query(const std::string &filename, benchmark_queries query) {
  // queries.
  std::cerr << "rdf4cpp: Parsing " << filename << std::endl;
  auto const mem_load_start = get_vmsize();
  auto const time_load_start = get_nanosec();
		rdf4cpp::Dataset dataset;
  auto g=dataset.graph();
  {
    using namespace rdf4cpp;
    for (const auto &e: rdf4cpp::parser::RDFFileParser{filename}) {
      if (e.has_value()) {
        auto const &quad = e.value();
        g.add(Statement{quad.subject(), quad.predicate(), quad.object()});
      } // else
				// there are thousands of invalid dates in the persondata file, ignore them
        //std::cerr << e.error();
    }
  }
  auto const time_load_done = get_nanosec();
  auto const duration_load = static_cast<double>(time_load_done - time_load_start) / 1e9;
  auto const mem_delta_load = get_vmsize() - mem_load_start; // difference may be 0 with 10k triples
  decltype(get_nanosec())
          time_query_first_result;

  ulong count = 0;
  // Enumerate all triples matching a pattern
  {
    using namespace rdf4cpp;
    using namespace rdf4cpp::query;
    auto triple_pattern = (query == benchmark_queries::query_1) ?
                          TriplePattern{Variable("x"), IRI{RDF_TYPE}, IRI{DBO_PERSON}} :
                          TriplePattern{IRI(DBR_VINCENT), Variable("y"), Variable("z")};
				Graph::solution_sequence solutions = g.match(triple_pattern);
    //std::cout << "g size " << g.size() << std::endl;
    for ([[maybe_unused]]const auto &solution: solutions) {
      if (count == 0) {
        time_query_first_result = get_nanosec();
      }
      count++;
    }
  }
  auto const time_query_done = get_nanosec();
  auto const duration_query_first = static_cast<double>(time_query_first_result - time_load_done) / 1e9;
  auto const duration_query_rest = static_cast<double>(time_query_done - time_query_first_result) / 1e9;
  std::cerr << "rdf4cpp: got " << count << "matching statements" << std::endl;
  printf("%f,%ld,%f,%f\n", duration_load, mem_delta_load, duration_query_first, duration_query_rest);
}

void parse(const std::string &filename) {
  std::cerr << "rdf4cpp: Parsing " << filename << std::endl;

  auto const mem_start = get_vmsize();
  auto const time_start = get_nanosec();
  ulong count = 0;
  {
    using namespace rdf4cpp;
    parser::ParsingFlags flags = parser::ParsingFlag::Turtle;
    for (const auto &e: parser::RDFFileParser{filename, flags}) {
      if (e.has_value()) {
        count++;
      } else
        std::cerr << e.error();
    }
  }
  auto const time_passed = static_cast<double>(get_nanosec() - time_start) / 1e9;
  auto const mem_delta = get_vmsize() - mem_start; // difference may be 0 with 10k triples

  std::cout << time_passed << std::endl;
  std::cerr << "rdf4cpp: parsed " << count << " triples" << std::endl;
  std::cerr << "rdf4cpp: mem delta " << mem_delta << std::endl;
}

int main(int argc, char *argv[]) {
  if (argc < 3) {
    fprintf(stderr, "Usage: %s <task> <filename> [options...]\n", argv[0]);
    return 1;
  }
  std::string const task{argv[1]};
  if (task == "parse") {
    parse(argv[2]);
  } else if (task == "query") {
    query(argv[2], benchmark_queries::query_1);
  } else if (task == "query2") {
    query(argv[2], benchmark_queries::query_2);
  } else {
    std::cerr << "Unknown task " << task << std::endl;
    return EXIT_FAILURE;
  }
  return EXIT_SUCCESS;
}
