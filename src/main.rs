use std::env;

use getopts::Options;
use scrypt::{ScryptParams, scrypt};
use atty::Stream;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("d", "domain", "domain (website / service)", "DOMAIN");
    opts.optopt("u", "user", "username", "USER");
    opts.optopt("r", "revision", "password revision", "REVISION");
    opts.optflag("h", "help", "print this help menu");
    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("h") ||
        !matches.opt_present("d") ||
        !matches.opt_present("u") {
        print_usage(&program, opts);
        return;
    }

    let domain = match matches.opt_str("d") {
        Some(s) => { s }
        None => {
            print_usage(&program, opts);
            return;
        }
    };
    let user = match matches.opt_str("u") {
        Some(s) => { s }
        None => {
            print_usage(&program, opts);
            return;
        }
    };

    // Is this how you concatenate strings?
    // Format as domain/username#revision where #revision is optional
    let mut salt = "".to_owned();
    salt.push_str(&domain);
    salt.push('/');
    salt.push_str(&user);

    match matches.opt_str("r") {
        Some(s) => {
            salt.push('#');
            salt.push_str(&s);
        }
        None => {}
    };

    let pass = rpassword::read_password_from_tty(Some("Secret:"))
    .unwrap()
    .to_owned();

    let pass_check = rpassword::read_password_from_tty(Some("Secret (confirm):"))
    .unwrap()
    .to_owned();

    if pass != pass_check {
        eprintln!("Secrets do not match");
        return;
    }

    // 15 is log2 of 32768
    let params = ScryptParams::new(15, 8, 4).unwrap();
    let mut out = [0u8; 18];

    scrypt(&pass.as_bytes(), &salt.as_bytes(), &params, &mut out).unwrap();

    let base64 = radix64::CustomConfig::with_alphabet(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_."
    )
    .no_padding()
    .build()
    .unwrap();

    let password = base64.encode(&out);

    if atty::is(Stream::Stdout) {
        eprintln!("{}", &password);
      } else {
        eprint!("{}", &password);
      }
}
