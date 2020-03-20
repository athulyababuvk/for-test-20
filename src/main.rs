extern crate clap;
use clap::{Arg, App};

mod read;
mod write;
mod today;
mod age_calculator;

fn main(){
    let matches = App::new("First Exercise")
                        .version("1.0")
                        .author("Athulya")
                        .about("Argument Passing")
                    .arg(Arg::with_name("input")
                            .short("i")
                            .long("input")
                            .takes_value(true))
                    .arg(Arg::with_name("output")
                            .short("o")
                            .long("output")
                            .takes_value(true))
        .get_matches();

        let input_file = matches.value_of("input").expect("Path not found");
        let output_file = matches.value_of("output".expect("Output file could not be created");
        let input = read::read(input_file);
        let mut output = write::write(output_file);
        age_calculator::age_calculator(input,&mut output);
}   
