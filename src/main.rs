extern crate flate2;
extern crate indicatif;

#[macro_use]
extern crate clap;
use clap::App;

use std::env;
use std::string::String;

use std::io::prelude::*;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::str;
use std::io::stdout;

const KMER: usize = 21;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    println!("{:#?}", matches);

    let gzip_output = matches.is_present("gzip");
    let k = value_t!(matches, "kmer", u8).unwrap_or(25);

    let output: std::io::BufWriter<Box<Write>> = match matches.value_of("output") {
        None           => {
                            BufWriter::new(Box::new(stdout()))
                        },
        Some(filename) => {
                            let output_fh = OpenOptions::new().append(true).create(true).open(filename).unwrap();
                            BufWriter::new(Box::new(output_fh))
                        }
    };

    // TODO: Put GzEncoder logic here if gzip flag is passed

    if let Some(fastas) = matches.values_of("fasta") {
        for fasta in fastas {
            parse_fasta(fasta);
        }
        
    }

}

// Should/could turn this into an iterator, but... eh
fn parse_fasta(filename: &str) {
    let gzipped: bool = filename.ends_with("gz");
    println!("{}", gzipped);

    let fasta_fh = match File::open(filename) {
        Ok(x) => x,
        Err(y) => panic!("Unable to open file: {} because {}", filename, y)
    };
  
    let fasta: Box<Read> = match filename.ends_with("gz") {
        true  => Box::new(flate2::read::GzDecoder::new(fasta_fh)),
        false => Box::new(fasta_fh)
    };

    let fasta = BufReader::with_capacity(64 * 1024 * 1024, fasta);

    for line in fasta.lines() {
        output_writer.write(labels.as_bytes());
        output_writer.write(b" ");
        for kmerx in line.unwrap().chars().collect::<Vec<char>>().chunks_exact(KMER).map(|x| x.into_iter().collect::<String>()) {
            let length = kmerx.chars().count();
            if length == KMER {
                output_writer.write(kmerx.as_bytes());
                output_writer.write(b" ");
            }
        }
        output_writer.write(b"\n");
    }


}






    

/*    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();
    let output_filename = args[2].clone();
    let mut labels: String = "".to_string();

    for label in args.iter().skip(3) {
        labels = format!("{} __label__{}", labels, label);
    }

    let output = OpenOptions::new().append(true).create(true).open(output_filename).unwrap();
    let mut output_writer = BufWriter::new(output);

    let fasta_fh = File::open(filename).unwrap();
    let fasta = flate2::read::GzDecoder::new(fasta_fh);
    let fasta = BufReader::with_capacity(64 * 1024 * 1024, fasta);

    for line in fasta.lines() {
        output_writer.write(labels.as_bytes());
        output_writer.write(b" ");
        for kmerx in line.unwrap().chars().collect::<Vec<char>>().chunks_exact(KMER).map(|x| x.into_iter().collect::<String>()) {
            let length = kmerx.chars().count();
            if length == KMER {
                output_writer.write(kmerx.as_bytes());
                output_writer.write(b" ");
            }
        }
        output_writer.write(b"\n");

    } 
} */
