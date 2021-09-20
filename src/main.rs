mod ean13;

static HELP: &str = "\
Usage: ean13       - Generate an EAN
       ean13 <EAN> - Validate the EAN
       ean13 -h    - Show help";

fn main() {
    let mut args = std::env::args();
    if args.len() > 2 {
        eprintln!("{}", HELP);
        std::process::exit(1);
    } else {
        match args.nth(1).as_deref() {
            Some("-h") => println!("{}", HELP),
            Some(arg) => {
                if ean13::is_valid(arg) {
                    println!("Valid EAN");
                } else {
                    println!("Invalid EAN");
                    std::process::exit(1);
                }
            }
            None => println!("{}", ean13::generate()),
        }
    }
}
