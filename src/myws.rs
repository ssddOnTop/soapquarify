use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::time::{SystemTime, UNIX_EPOCH};
use encrypto_sha256::ZotSha256;
use reqwest::Client;
use serde_json::Value;
use ws::{CloseCode, connect, Handshake, listen, Message, Sender};
use crate::mhf::MHF;
use crate::stj::STJ;

pub struct WS {
    sender: Sender,
    mp: Arc<Mutex<HashMap<String, String>>>,
    path: String,
    pw: String
}

static mut TIM: u128 = 0;

impl ws::Handler for WS {
    fn on_open(&mut self, shake: Handshake) -> ws::Result<()> {
        // self.sender.send(self.aes.get_sterilised_key()).unwrap();
        Ok(())
    }
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        let sender = self.sender.clone();
        // let mut aes = self.aes.clone();
        let mut mp = Arc::clone(&self.mp);
        let dir = self.path.clone();
        let pw = self.pw.clone();
        spawn(move || {
            let mut mp = mp.lock().unwrap();
            let msg = msg.as_text().unwrap();
            // let msg = aes.decrypt_cbc(msg.to_string());
            let x: Value = match serde_json::from_str(msg) {
                Ok(v) => v,
                _ => {
                    sender.send("parse err").unwrap();
                    Value::default()
                }
            };
            if x == Value::default() {
                return ();
            }
            let id = x["id"].as_str().unwrap();

            match id {
                "xml" => {
                    if x["pw"] == pw {
                        let mut fil = File::create(format!("{}/file.xml", dir)).unwrap();

                        /// check if given xml is valid. if it's valid, it's stored else client is responded with Invalid xml.
                        match serde_xml_rs::from_str::<Value>(x["value"].as_str().unwrap()) {
                           Ok(_) => {
                               fil.write_all(x["value"].as_str().unwrap().as_bytes()).unwrap();
                               sender.send("Ok").unwrap();
                           },
                            Err(_) => {
                                sender.send("Invalid xml").unwrap();
                            }
                        }
                    } else {
                        sender.send("Invalid password").unwrap();
                        sender.close(CloseCode::Invalid).unwrap()
                    }
                }
                "qry" => {
                    /// Cache is cleared every 2 mins.
                    unsafe {
                        if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - TIM >= 120000 {
                            mp.clear();
                            TIM = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                        }
                    }
                    let path = x["path"].as_str().unwrap();
                    let zsha = ZotSha256::encode_to_base64(path.as_bytes());
                    if let Some(val) = mp.get(&*zsha) {
                        sender.send(val.as_str()).unwrap();
                    } else {
                        let mut fil = File::open(format!("{}/file.xml", dir)).unwrap();
                        let mut st = "".to_string();
                        fil.read_to_string(&mut st).unwrap();
                        let mut obj: Value = serde_xml_rs::from_str(&*st).unwrap();
                        let sp: Vec<&str> = path.split('/').filter(|x| !x.is_empty()).collect();
                        for c in sp {
                            if !obj[c].is_null() {
                                obj = obj[c].clone();
                            } else {
                                sender.send("Invalid path.").unwrap();
                                return ();
                            }
                        }
                        let obj = STJ::xtj_fromval(obj).unwrap().to_string();
                        sender.send(obj.clone()).unwrap();
                        mp.insert(zsha, obj);
                    }
                }
                _ => {
                    sender.send("Invalid id").unwrap();
                }
            }
        });
        Ok(())
    }
}

impl WS {
    pub fn init(link: &str, path: String,pw:String) {
        listen(link, |out| {
            WS {
                sender: out,
                mp: Arc::new(Mutex::new(HashMap::new())),
                path: path.clone(),
                pw: pw.clone(),
            }
        }).unwrap();
    }
}