use std::{env, fs, io, process};
use std::io::Write;
use std::str::FromStr;

extern crate regex;
use regex::Regex;

extern crate time;
use time::OffsetDateTime;

extern crate sophia;
use sophia::graph::*;
use sophia::graph::inmem::*;
use sophia::ns::rdf;
use sophia::parser::nt;
use sophia::triple::stream::*;
use sophia::term::*;
extern crate hdt;
use hdt::{HdtGraph,Hdt};


fn get_vmsize() -> usize {
    let status = fs::read_to_string("/proc/self/status").unwrap();
    //let vmsize_re = Regex::new(r"VmSize:\s*([0-9]+) kB").unwrap();
    let vmsize_re = Regex::new(r"VmRSS:\s*([0-9]+) kB").unwrap();
    let vmsize = vmsize_re.captures(&status).unwrap().get(1).unwrap().as_str();
    usize::from_str(vmsize).unwrap()
}

fn task_query (filename: &str, variant: Option<&str>)
{
    eprintln!("task    : query");
    match variant {
        None => { 
             let f = fs::File::open(&filename).expect("Error opening file");
            let f = io::BufReader::new(f);
            task_query_g(f, FastGraph::new(), 1);
        }
        Some("light") => {
            let f = fs::File::open(&filename).expect("Error opening file");
            let f = io::BufReader::new(f);
            task_query_g(f, LightGraph::new(), 1);
        }
        Some("hdt") => {
            let f = fs::File::open(&filename.replace("ttl","hdt")).expect("Error opening file");
            let f = io::BufReader::new(f);
            task_query_hdt(f, 1);
        }
        Some(v) => {
            eprintln!("Unknown variant {}", v);
            process::exit(1);
        }
    };
}

fn task_query2(filename: &str, variant: Option<&str>)
{
    let f = fs::File::open(&filename).expect("Error opening file");
    let f = io::BufReader::new(f);
    eprintln!("task    : query2");
    match variant {
        None => {
            task_query_g(f, FastGraph::new(), 2);
        }
        Some("light") => {
            task_query_g(f, LightGraph::new(), 2);
        }
        Some(v) => {
            eprintln!("Unknown variant {}", v);
            process::exit(1);
        }
    };
}

fn task_query_hdt<R> (f: R, query_num: usize) where
    R: io::BufRead,
{
    let m0 = get_vmsize();
    let t0 = OffsetDateTime::now();
    let hdt = Hdt::new(std::io::BufReader::new(f)).expect("error loading HDT");
    let g = HdtGraph::new(hdt);
    let t1 = OffsetDateTime::now();
    let m1 = get_vmsize();
    let time_parse = (t1-t0).as_seconds_f64();
    let mem_graph = m1-m0;
    eprintln!("loaded  : ~ {:?} triples\n", g.triples().size_hint());

    let mut time_first: f64 = 0.0;
    let time_rest;
    let dbo_person = BoxTerm::new_iri_unchecked("http://dbpedia.org/ontology/Person".to_owned());
    let dbr_vincent = BoxTerm::new_iri_unchecked("http://dbpedia.org/resource/Vincent_Descombes_Sevoie".to_owned());
    let mut t0 = OffsetDateTime::now();
    let results = match query_num {
        1 => g.triples_with_po(&rdf::type_, &dbo_person),
        _ => g.triples_with_s(&dbr_vincent),
    };

    let mut c = 0;
    for _ in results {
        if c == 0 {
            let t1 = OffsetDateTime::now();
            time_first = (t1-t0).as_seconds_f64();
            t0 = OffsetDateTime::now();
        }
        c += 1;
    }
    let t1 = OffsetDateTime::now();
    time_rest = (t1-t0).as_seconds_f64();
    eprintln!("matching triple: {}\n", c);

    println!("{},{},{},{}", time_parse, mem_graph, time_first, time_rest);
}

fn task_query_g<G, R> (f: R, mut g: G, query_num: usize) where
    R: io::BufRead,
    G: MutableGraph,
{
    let m0 = get_vmsize();
    let t0 = OffsetDateTime::now();
    g.insert_all(nt::parse_bufread(f)).expect("Error parsing NT file");
    let t1 = OffsetDateTime::now();
    let m1 = get_vmsize();
    let time_parse = (t1-t0).as_seconds_f64();
    let mem_graph = m1-m0;
    eprintln!("loaded  : ~ {:?} triples\n", g.triples().size_hint());

    let mut time_first: f64 = 0.0;
    let time_rest;
    let dbo_person = Term::<&'static str>::new_iri("http://dbpedia.org/ontology/Person").unwrap();
    let dbr_vincent = Term::<&'static str>::new_iri("http://dbpedia.org/resource/Vincent_Descombes_Sevoie").unwrap();

    let mut t0 = OffsetDateTime::now();
    let results = match query_num {
        1 => g.triples_with_po(&rdf::type_, &dbo_person),
        _ => g.triples_with_s(&dbr_vincent),
    };

    let mut c = 0;
    for _ in results {
        if c == 0 {
            let t1 = OffsetDateTime::now();
            time_first = (t1-t0).as_seconds_f64();
            t0 = OffsetDateTime::now();
        }
        c += 1;
    }
    let t1 = OffsetDateTime::now();
    time_rest = (t1-t0).as_seconds_f64();
    eprintln!("matching triple: {}\n", c);

    println!("{},{},{},{}", time_parse, mem_graph, time_first, time_rest);
}

fn task_parse (filename: &str, variant: Option<&str>) {
    eprintln!("task    : parse");
    match variant {
        None => {
            task_parse_nt(filename);
        }
        Some("hdt") => {
            task_parse_hdt(filename);
        }
        Some(v) => {
            eprintln!("Unknown variant {}", v);
            process::exit(1);
        }
    };
}

fn task_parse_nt (filename: &str) {
    let f = fs::File::open(&filename).expect("Error opening file");
    let f = io::BufReader::new(f);
    let t0 = OffsetDateTime::now();
    nt::parse_bufread(f).for_each_triple(|_| ()).expect("Error parsing NT file");
    let t1 = OffsetDateTime::now();
    let time_parse = (t1-t0).as_seconds_f64();
    println!("{}", time_parse);
}

fn task_parse_hdt (filename: &str) {
    let f = fs::File::open(&filename.replace("ttl","hdt")).expect("Error opening file");
    let f = io::BufReader::new(f);
    let t0 = OffsetDateTime::now();
    hdt::Hdt::new(f).unwrap();
    //t::parse_bufread(f).for_each_triple(|_| ()).expect("Error parsing NT file");
    let t1 = OffsetDateTime::now();
    let time_parse = (t1-t0).as_seconds_f64();
    println!("{}", time_parse);
}


fn main() {
    eprintln!("program : sophia");
    eprintln!("pid     : {}", process::id());
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        io::stderr().write(b"usage: sophia_benchmark <task> <filename.nt>\n").unwrap();
        process::exit(1);
    }
    let task_id: &str = &args[1];
    let filename = &args[2];
    let variant = if args.len() > 3 {
        Some(&args[3] as &str)
    } else {
        None
    };
    eprintln!("filename: {}", filename);
    match task_id {
        "parse"  => task_parse(filename, variant),
        "query" => task_query(filename, variant),
        "query2" => task_query2(filename, variant),
        _   => {
            eprint!("Unknown task {}", task_id);
            process::exit(1);
        }
    };
}
