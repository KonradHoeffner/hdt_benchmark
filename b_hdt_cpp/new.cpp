#include <iostream>
#include <HDTManager.hpp>
	
using namespace std;
using namespace hdt;
	
int main(int argc, char *argv[]) {
				// Load HDT file (Use mapIndexedHDT if you plan to use ?p?, ?po or ??o queries.
				HDT *hdt = HDTManager::mapHDT("data/test.hdt");
	
				// Enumerate all triples matching a pattern ("" means any)
				IteratorTripleString *it = hdt->search("http://example.org/uri3","","");
				while(it->hasNext()){
								TripleString *triple = it->next();
								cout << triple->getSubject() <<
								", " << triple->getPredicate() <<
								", " << triple->getObject() << endl;
				}
				delete it; // Remember to delete iterator to avoid memory leaks!
				delete hdt; // Remember to delete instance when no longer needed!
}
