import java.io.*;
import java.nio.file.Path;

import java.util.Iterator;

import org.rdfhdt.hdt.hdt.HDT;
import org.rdfhdt.hdt.hdt.HDTManager;
import org.rdfhdt.hdt.triples.IteratorTripleString;
import org.rdfhdt.hdt.triples.TripleString;

public class HdtJavaBenchmark {
    public static void main(String[] args) throws Exception {
        // configure Log4J (to get rid of warn messages)
        //PropertyConfigurator.configure("apache-jena-4.6.1/log4j2.properties");

        if (args[0].equals("parse")) {
            benchmark_parse(args);
        } else if (args[0].equals("query")) {
            benchmark_query(1, args);
        } else if (args[0].equals("query2")) {
            benchmark_query(2, args);
        } else if (args[0].equals("test")) {
            benchmark_test(args);
        } else {
            System.err.println("Unrecognized benchmark name");
        }
    }

    public static void benchmark_parse(String[] args) throws Exception {
        System.err.println("benchmark: parse");
        final long t0 = System.nanoTime();
        HDT hdt = HDTManager.loadIndexedHDT(args[1].replaceAll("ttl","hdt"), null);
        hdt.close();
        final long t1 = System.nanoTime();
        final double diff = (t1 - t0)/1e9;
        System.out.println(diff);
    }

    public static void benchmark_query(int queryNum, String[] args) throws Exception {
        // writes 3 numbers:
        // - time (in s) to load the NT file into an in-memory graph
        // - memory (in kB) allocated for creating and loading graph
        // - time (in s) to retrieve the first triple matching (* rdf:type *)
        // - time (in s) to retrieve all the remaining matching triples
        System.err.println("benchmark: query");

        double time_load, time_first = 0, time_rest;
        long mem_graph;

        long m0, m1;
        m0 = get_memory_footprint();

        long t0, t1;
        t0 = System.nanoTime();
		// HDT Java writes directly to stdout so this is the only way we can turn it off
		PrintStream original = System.out;
		System.setOut(new PrintStream(new FileOutputStream("/dev/null")));
        HDT hdt = HDTManager.loadIndexedHDT(args[1].replaceAll("ttl","hdt"), null);
		System.setOut(original);
		System.err.println(args[1].replaceAll("ttl","hdt"));
        t1 = System.nanoTime();
        m1 = get_memory_footprint();
        time_load = (t1 - t0) / 1e9;
        mem_graph = m1 - m0;
        t0 = System.nanoTime();
        String personClass = "http://dbpedia.org/ontology/Person";
        String vincent = "http://dbpedia.org/resource/Vincent_Descombes_Sevoie";
        String rdfType = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"; 
        long nb_stmts = 0;
        
		try {
            IteratorTripleString it;
			//= hdt.search("", "", "");
            if (queryNum == 1) {
                it = hdt.search("", rdfType, personClass);
            } else {// if (queryNum == 2) {
                it = hdt.search(vincent, "", "");
            }
            
            while(it.hasNext()) {
                TripleString ts = it.next();
                nb_stmts += 1;
                if (nb_stmts == 1) {
                    t1 = System.nanoTime();
                    time_first = (t1 - t0) / 1e9;
                    t0 = System.nanoTime();
                }
                //System.out.println(ts);
            }
        } finally {
            hdt.close();
        }

        t1 = System.nanoTime();
        time_rest = (t1 - t0) / 1e9;

        //System.err.println("parsed: " + model.size() + " statements");
        System.err.println("matched: " + nb_stmts + " statements");
        System.out.println(time_load + "," + mem_graph + "," + time_first + "," + time_rest);
    }

    public static void benchmark_test(String[] args) {
        System.err.println("benchmark: test");
    }

    public static long get_memory_footprint() {
        try {
            String filename = "/proc/" + ProcessHandle.current().pid() + "/status";
            BufferedReader br;
            br = new BufferedReader(
                    new InputStreamReader(
                        new FileInputStream(filename)));
            String vmsize = br.lines()
                .filter(line -> line.matches("VmRSS.*"))
                .findFirst()
                .get()
                .replaceAll("VmRSS:\\h*", "")
                .replaceAll(" *kB", "");
            return Long.parseLong(vmsize);
        }
        catch (Exception ex) {
            throw new RuntimeException(ex);
        }
    }

}
