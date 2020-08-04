use clap::{App, Arg};
use purr::core::*;
use purr::graphics::*;

fn main() {
    let matches = App::new("Purr")
        .version("0.0")
        .author("Yongsheng Xu")
        .about("Rusty Days Hackathon Project")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .help("Input Image")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Output Image")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("number of shapes")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let shape_number = matches.value_of("number").unwrap().parse().unwrap();
    let mut runner: PurrModelRunner<Triangle> = PurrModelRunner::new(shape_number, 4);
    let model_ctx = PurrContext::new(input);
    let mut model = PurrModel::new(model_ctx, 1000, 16, 100);
    runner.run(&mut model, output);
}
