use std::io::Write;
use std::str::FromStr;
use std::{env, fs, io, process};
use std::rc::Rc;

extern crate regex;
use regex::Regex;

extern crate time;
use time::OffsetDateTime;

extern crate sophia;
use sophia::graph::inmem::*;
use sophia::graph::*;
use sophia::ns::rdf;
use sophia::parser::nt;
use sophia::term::*;
use sophia::triple::stream::*;
extern crate hdt;
use hdt::{Hdt, HdtGraph, IdKind};

fn get_vmsize() -> usize {
    let status = fs::read_to_string("/proc/self/status").unwrap();
    //let vmsize_re = Regex::new(r"VmSize:\s*([0-9]+) kB").unwrap();
    let vmsize_re = Regex::new(r"VmRSS:\s*([0-9]+) kB").unwrap();
    let vmsize = vmsize_re
        .captures(&status)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    usize::from_str(vmsize).unwrap()
}

fn task_query(filename: &str, variant: Option<&str>, query_num: usize) {
    eprintln!("task    : query");
    match variant {
        None => {
            let f = fs::File::open(&filename).expect("Error opening file");
            let f = io::BufReader::new(f);
            task_query_g(f, FastGraph::new(), query_num);
        }
        Some("light") => {
            let f = fs::File::open(&filename).expect("Error opening file");
            let f = io::BufReader::new(f);
            task_query_g(f, LightGraph::new(), query_num);
        }
        Some("sophia_hdt") => {
            let f = fs::File::open(&filename.replace("ttl", "hdt")).expect("Error opening file");
            let f = io::BufReader::new(f);
            task_query_sophia_hdt(f, query_num);
        }
        Some("hdt") => {
            let f = fs::File::open(&filename.replace("ttl", "hdt")).expect("Error opening file");
            let f = io::BufReader::new(f);
            task_query_hdt(f, query_num);
        }
        Some(v) => {
            eprintln!("Unknown variant {}", v);
            process::exit(1);
        }
    };
}

fn task_query_hdt<R>(f: R, query_num: usize)
where
    R: io::BufRead,
{
    let m0 = get_vmsize();
    let t0 = OffsetDateTime::now_utc();
    let hdt = Hdt::<Rc<str>>::new(std::io::BufReader::new(f)).expect("error loading HDT");
    let t1 = OffsetDateTime::now_utc();
    let m1 = get_vmsize();
    let time_parse = (t1 - t0).as_seconds_f64();
    let mem_graph = m1 - m0;
    eprintln!("loaded {hdt:?}");

    let mut time_first: f64 = 0.0;
    let time_rest;
    let dbo_person = "http://dbpedia.org/ontology/Person";
    let dbr_vincent = "http://dbpedia.org/resource/Vincent_Descombes_Sevoie";
    let dbo_gender = "http://dbpedia.org/ontology/gender";
    let dbr_paris = "http://dbpedia.org/resource/Paris";
    let dbo_birthplace = "http://dbpedia.org/ontology/birthPlace";
    let male = "male@\"en\"";
    let female = "female@\"en\"";
    let queer = "genderqueer@\"en\"";
    let mut t0 = OffsetDateTime::now_utc();
    let type_ = rdf::type_.value();
    let results = match query_num {
        1 => Box::new(hdt.triples_with_po(&type_, dbo_person)),
        2 => hdt.triples_with(&dbr_vincent, &IdKind::Subject),
        3 => Box::new(hdt.triples_with_sp(&dbr_vincent, &type_)),
        4 => hdt.triples_with(&dbo_gender, &IdKind::Predicate),
        5 => hdt.triples_with(&dbo_birthplace, &IdKind::Predicate),
        6 => hdt.triples_with(&male, &IdKind::Object),
        7 => hdt.triples_with(&female, &IdKind::Object),
        8 => hdt.triples_with(&dbr_paris, &IdKind::Object),
        9 => Box::new(hdt.triples_with_po(&dbo_birthplace, &dbr_paris)),
        10 => Box::new(hdt.triples_with_po(&dbo_gender, &male)),
        11 => Box::new(hdt.triples_with_po(&dbo_gender, &female)),
        12 => Box::new(hdt.triples_with_po(&dbo_gender, &queer)),
        13 => Box::new(hdt.triples_with_so(&dbr_vincent, &dbo_person)),
        _ => panic!("Unknown query num {query_num}."),
    };

    let mut c = 0;
    for _ in results {
        if c == 0 {
            let t1 = OffsetDateTime::now_utc();
            time_first = (t1 - t0).as_seconds_f64();
            t0 = OffsetDateTime::now_utc();
        }
        c += 1;
    }
    let t1 = OffsetDateTime::now_utc();
    time_rest = (t1 - t0).as_seconds_f64();
    eprintln!("matching triple: {}\n", c);

    println!("{},{},{},{}", time_parse, mem_graph, time_first, time_rest);
}

fn task_query_sophia_hdt<R>(f: R, query_num: usize)
where
    R: io::BufRead,
{
    let m0 = get_vmsize();
    let t0 = OffsetDateTime::now_utc();
    let hdt = Hdt::<Rc<str>>::new(std::io::BufReader::new(f)).expect("error loading HDT");
    let g = HdtGraph::new(hdt);
    let t1 = OffsetDateTime::now_utc();
    let m1 = get_vmsize();
    let time_parse = (t1 - t0).as_seconds_f64();
    let mem_graph = m1 - m0;
    //eprintln!("loaded  : ~ {:?} triples\n", g.triples().size_hint());

    let mut time_first: f64 = 0.0;
    let time_rest;
    /*
    1414218 <http://dbpedia.org/ontology/Person> .
    1180655 "male"@en .
     236862 "female"@en .
      36675 "John"@en .
      32669 "American politician"@en .
      20851 "William"@en .
      18111 <http://dbpedia.org/resource/Paris> .
      16285 <http://dbpedia.org/resource/New_York_City> .
      15748 "James"@en .
      15431 <http://dbpedia.org/resource/London> .
    */
    let dbo_person = BoxTerm::new_iri_unchecked("http://dbpedia.org/ontology/Person".to_owned());
    let dbr_vincent = BoxTerm::new_iri_unchecked(
        "http://dbpedia.org/resource/Vincent_Descombes_Sevoie".to_owned(),
    );
    let dbo_gender = BoxTerm::new_iri_unchecked("http://dbpedia.org/ontology/gender".to_owned());
    let dbr_paris = BoxTerm::new_iri_unchecked("http://dbpedia.org/resource/Paris".to_owned());
    let dbo_birthplace =
        BoxTerm::new_iri_unchecked("http://dbpedia.org/ontology/birthPlace".to_owned());
    let male = BoxTerm::new_literal_lang_unchecked("male", "en");
    let female = BoxTerm::new_literal_lang_unchecked("female", "en");
    let queer = BoxTerm::new_literal_lang_unchecked("genderqueer", "en");
    let mut t0 = OffsetDateTime::now_utc();
    let results = match query_num {
        1 => g.triples_with_po(&rdf::type_, &dbo_person),
        2 => g.triples_with_s(&dbr_vincent),
        3 => g.triples_with_sp(&dbr_vincent, &rdf::type_),
        4 => g.triples_with_p(&dbo_gender),
        5 => g.triples_with_p(&dbo_birthplace),
        6 => g.triples_with_o(&male),
        7 => g.triples_with_o(&female),
        8 => g.triples_with_o(&dbr_paris),
        9 => g.triples_with_po(&dbo_birthplace, &dbr_paris),
        10 => g.triples_with_po(&dbo_gender, &male),
        11 => g.triples_with_po(&dbo_gender, &female),
        12 => g.triples_with_po(&dbo_gender, &queer),
        13 => g.triples_with_so(&dbr_vincent, &dbo_person),
        _ => panic!("Unknown query num {query_num}."),
    };

    let mut c = 0;
    for _ in results {
        if c == 0 {
            let t1 = OffsetDateTime::now_utc();
            time_first = (t1 - t0).as_seconds_f64();
            t0 = OffsetDateTime::now_utc();
        }
        c += 1;
    }
    let t1 = OffsetDateTime::now_utc();
    time_rest = (t1 - t0).as_seconds_f64();
    eprintln!("matching triple: {}\n", c);

    println!("{},{},{},{}", time_parse, mem_graph, time_first, time_rest);
}

fn task_query_g<G, R>(f: R, mut g: G, query_num: usize)
where
    R: io::BufRead,
    G: MutableGraph,
{
    let m0 = get_vmsize();
    let t0 = OffsetDateTime::now_utc();
    g.insert_all(nt::parse_bufread(f))
        .expect("Error parsing NT file");
    let t1 = OffsetDateTime::now_utc();
    let m1 = get_vmsize();
    let time_parse = (t1 - t0).as_seconds_f64();
    let mem_graph = m1 - m0;
    //eprintln!("loaded  : ~ {:?} triples\n", g.triples().size_hint());

    let mut time_first: f64 = 0.0;
    let time_rest;
    let dbo_person = Term::<&'static str>::new_iri("http://dbpedia.org/ontology/Person").unwrap();
    let dbr_vincent =
        Term::<&'static str>::new_iri("http://dbpedia.org/resource/Vincent_Descombes_Sevoie")
            .unwrap();

    let mut t0 = OffsetDateTime::now_utc();
    let results = match query_num {
        1 => g.triples_with_po(&rdf::type_, &dbo_person),
        _ => g.triples_with_s(&dbr_vincent),
    };

    let mut c = 0;
    for _ in results {
        if c == 0 {
            let t1 = OffsetDateTime::now_utc();
            time_first = (t1 - t0).as_seconds_f64();
            t0 = OffsetDateTime::now_utc();
        }
        c += 1;
    }
    let t1 = OffsetDateTime::now_utc();
    time_rest = (t1 - t0).as_seconds_f64();
    eprintln!("matching triple: {}\n", c);

    println!("{},{},{},{}", time_parse, mem_graph, time_first, time_rest);
}

fn task_parse(filename: &str, variant: Option<&str>) {
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

fn task_parse_nt(filename: &str) {
    let f = fs::File::open(&filename).expect("Error opening file");
    let f = io::BufReader::new(f);
    let t0 = OffsetDateTime::now_utc();
    nt::parse_bufread(f)
        .for_each_triple(|_| ())
        .expect("Error parsing NT file");
    let t1 = OffsetDateTime::now_utc();
    let time_parse = (t1 - t0).as_seconds_f64();
    println!("{}", time_parse);
}

fn task_parse_hdt(filename: &str) {
    let f = fs::File::open(&filename.replace("ttl", "hdt")).expect("Error opening file");
    let f = io::BufReader::new(f);
    let t0 = OffsetDateTime::now_utc();
    hdt::Hdt::<Rc<str>>::new(f).unwrap();
    //t::parse_bufread(f).for_each_triple(|_| ()).expect("Error parsing NT file");
    let t1 = OffsetDateTime::now_utc();
    let time_parse = (t1 - t0).as_seconds_f64();
    println!("{}", time_parse);
}

fn main() {
    eprintln!("program : sophia");
    eprintln!("pid     : {}", process::id());
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        io::stderr()
            .write(b"usage: sophia_benchmark <task> <filename.nt>\n")
            .unwrap();
        process::exit(1);
    }
    let mut task_id: &str = &args[1];
    let filename = &args[2];
    let variant = if args.len() > 3 {
        Some(&args[3] as &str)
    } else {
        None
    };
    eprintln!("filename: {}", filename);
    let mut query_num = 1;
    if task_id.starts_with("query") && task_id.len() > 5 {
        query_num = task_id
            .split("query")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        task_id = "query";
    }
    match task_id {
        "parse" => task_parse(filename, variant),
        "query" => task_query(filename, variant, query_num),
        _ => {
            eprint!("Unknown task {}", task_id);
            process::exit(1);
        }
    };
}
