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
        .arg(
            Arg::with_name("thread")
                .short("t")
                .long("thread")
                .help("numebr of threads")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("shape")
                .short("s")
                .long("shape")
                .help("shape: triangle/ellipse")
                .takes_value(true),
        )
        .get_matches();
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let shape_number = matches.value_of("number").unwrap().parse().unwrap();
    let shape = matches.value_of("shape").unwrap_or("triangle");
    let thread_number = matches
        .value_of("thread")
        .unwrap_or(&num_cpus::get().to_string())
        .parse()
        .unwrap();
    let model_ctx = PurrContext::new(input);
    match shape {
        "triangle" => {
            let mut runner = PurrModelRunner::<Triangle>::new(shape_number, thread_number);
            let mut model = PurrHillClimbModel::new(model_ctx, 1000, 16, 100);
            runner.run(&mut model, output);
        }
        "ellipse" => {
            let mut runner = PurrModelRunner::<Ellipse>::new(shape_number, thread_number);
            let mut model = PurrHillClimbModel::new(model_ctx, 1000, 16, 100);
            runner.run(&mut model, output);
        }
        _ => {
            eprintln!("unsupported shape {}", shape);
            return;
        }
    };
}
