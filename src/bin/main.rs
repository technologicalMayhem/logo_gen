use std::env::args;

fn main() {
    let mut args = args();
    args.next();

    let Some(arg) = args.next() else {
        println!("No width provided.");
        return;
    };

    let Ok(width) = arg.parse::<f32>() else {
        println!("Failed to parse {arg} as float.");
        return;
    };

    let logo_svg = logo_gen::generate_logo(width);
    println!("{logo_svg}");
}