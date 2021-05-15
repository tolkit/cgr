use bio::io::fasta;
use cgr::cgr::cgr::generate_cgr;
use cgr::plot::plot::plot;
use cgr::write::write::write;
use clap::{value_t, App, Arg};
use std::fs::create_dir_all;

fn main() {
    // command line options
    let matches = App::new("cgr")
        .version(clap::crate_version!())
        .author("Max Brown <mb39@sanger.ac.uk>")
        .about("Make chaos game representations of a fasta file.")
        .arg(
            Arg::with_name("fasta")
                .short("f")
                .long("fasta")
                .takes_value(true)
                .required(true)
                .help("The fasta file"),
        )
        .arg(
            Arg::with_name("save")
                .short("s")
                .long("save")
                .takes_value(true)
                .possible_values(&["true", "false"])
                .required(false)
                .default_value("false")
                .help("Should the matrix coordinates be saved? Warning: these files can be very large, and take a while to write."),
        )
        .get_matches();
    // parse command line options
    let input_fasta = matches.value_of("fasta").unwrap();
    let save = value_t!(matches.value_of("save"), bool).unwrap_or_else(|e| e.exit());

    // create some sub-folders for output
    if save {
        if let Err(e) = create_dir_all("./cgr_out/data/") {
            println!("[-]\tCreate directory error: {}", e.to_string());
        }
    }
    if let Err(e) = create_dir_all("./cgr_out/images/") {
        println!("[-]\tCreate directory error: {}", e.to_string());
    }

    // read in the fasta from file
    let reader = fasta::Reader::from_file(input_fasta).expect("[-]\tPath invalid.");
    for result in reader.records() {
        let record = result.expect("[-]\tError during fasta record parsing.");
        // get length for dim calculation
        // must be a better and more clever way to get the best dims.
        let len = record.seq().len() as f64;
        let dim = (len.sqrt().round() + 250.0) as u32;

        // generate the matrix
        let cgr = generate_cgr(record.seq());
        // save
        if save {
            let data_file_name = format!("{}", record.id());
            write(&data_file_name, cgr.clone())
                .unwrap_or_else(|_| println!("[-]\tError in writing to file."));
            println!("{}.tsv written to file", record.id());
        }
        // save the PNG
        let file_name = format!("./cgr_out/images/{}.png", record.id());
        plot(cgr, &file_name, dim).expect("File did not write :(");
        println!("{}.png written to file", record.id());
    }
}
