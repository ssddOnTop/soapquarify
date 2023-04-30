use std::error;
use std::fs::File;
use std::io::Read;
use std::string::ToString;
use clap::arg;
use keyring::Entry;
use serde_json::Value;
use ws::Handler;
use crate::myws::WS;
use crate::stj::STJ;

mod stj;
mod mytests;
mod myws;
mod mhf;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub static mut TOTPS: String = String::new();

fn main() {
    /// Take args
    let matches = clap::Command::new("stj")
        .version("0.1.0")
        .author("Sandipsinh. <sandip@ssdd.dev>")
        .about("Servers the queries for pvtyt")
        .arg(arg!(-d --dir <VALUE>).required(true))
        .arg(arg!(-p --pw <VALUE>).required(false))
        // .arg(fp)
        .get_matches();

    /// Open and Read dir.
    let dir = matches.get_one::<String>("dir").expect("conf argument not found.");

    /// Check if there exists password in keychain. If there doesn't exist, it reads <path>/pw.txt and set the text as password.
    let entry = Entry::new("stj", "ssdd").unwrap();
    let mut pw = String::new();
    match entry.get_password() {
        Ok(pwx) => {
            pw = pwx;
        },
        Err(err) => {
            let mut v = File::open(matches.get_one::<String>("pw").expect("password file argument not found.")).unwrap();
            let mut st = String::new();
            v.read_to_string(&mut st).unwrap();
            entry.set_password(&*st).unwrap();
            pw = st;
        }
    };

    /// init websocket server
    WS::init("127.0.0.1:19194", dir.clone(),pw);
}

/*
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Raw("TestSecretSuperSecret".as_bytes().to_vec()).to_bytes().unwrap(),
    ).unwrap();
    let token = totp.generate_current().unwrap();

    println!("{}",token);

    let seconds: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let result = totp_lite::totp::<Sha1>(token.as_bytes(), seconds);
    println!("{}",result);
*/
