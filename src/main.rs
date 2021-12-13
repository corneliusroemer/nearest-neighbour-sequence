extern crate clap;
extern crate csv;
use clap::{App, Arg};
use std::collections::HashSet;
use std::io;

fn main() {
    let matches = App::new("Calculate distance from comparator sequence")
        .arg(
            Arg::with_name("COMPARATOR")
                .help("Sets the comparison file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(2),
        )
        .get_matches();

    let comparator = matches.value_of("COMPARATOR").unwrap();
    eprintln!("Value for comparator: {}", comparator);

    let input = matches.value_of("INPUT").unwrap();
    eprintln!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Parse comparator file
    let mut rdr_comp = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(comparator)
        .unwrap();
    let headers_comp = rdr_comp.headers().unwrap();
    eprintln!("Headers: {:?}", headers_comp);
    let mut firstline = csv::StringRecord::new();
    for result in rdr_comp.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here..
        firstline = result.unwrap();
        break;
    }
    let comp_muts = firstline.get(1).unwrap();

    let mut comp_muts_set = HashSet::new();

    comp_muts.split(',').for_each(|m| {
        comp_muts_set.insert(m.to_string());
    });
    eprintln!("Comparator mutations: {:?}", comp_muts_set);

    // Open writer
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::Never)
        .from_writer(io::stdout());

    // Read input file
    let mut rdr_inp = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(input)
        .unwrap();
    let headers_inp = rdr_inp.headers().unwrap();
    eprintln!("Headers: {:?}", headers_inp);
    let mut line;
    let mut inp_muts_set = HashSet::new();
    let mut count = 0;
    for result in rdr_inp.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here..
        line = result.unwrap();
        let inp_muts = line.get(1).unwrap();
        inp_muts_set.clear();
        inp_muts.split(',').for_each(|m| {
            inp_muts_set.insert(m.to_string());
        });
        let joint_muts_count = comp_muts_set.intersection(&inp_muts_set).count();
        wtr.write_record(&[line.get(0).unwrap(), &joint_muts_count.to_string()])
            .unwrap();
        if count % 10000 == 0 {
            eprintln!("Processed {} lines", count);
        }
        count += 1;
    }
}
