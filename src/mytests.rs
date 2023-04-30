use std::mem::size_of;
use serde_json::Value;

// These are tests, ignore the files.

#[cfg(test)]
mod Idek {
    use serde_json::Value;
    use crate::stj;
    use crate::mytests::{any_as_u8_slice, loopx};
    use std::cmp::min;
    use std::collections::HashMap;
    use num::One;
    use num_bigint::{BigUint, RandBigInt};
    use num_traits::ToPrimitive;
    use rand::thread_rng;
    use crate::mhf::MHF;
    #[test]
    fn testsop_to_json() {
        let xml = r#"
    <?xml version = "1.0"?>
<SOAP-ENV:Envelope
   xmlns:SOAP-ENV = "http://www.w3.org/2001/12/soap-envelope"
   SOAP-ENV:encodingStyle = "http://www.w3.org/2001/12/soap-encoding">

   <SOAP-ENV:Body xmlns:m = "http://www.xyz.org/quotations">
      <m:GetQuotation>
         <m:QuotationsName>MiscroSoft</m:QuotationsName>
      </m:GetQuotation>
   </SOAP-ENV:Body>
</SOAP-ENV:Envelope>
    "#;
        let mut x:Value = serde_xml_rs::from_str(xml).unwrap();
        loopx(&mut x);
        println!("{}",x);
    }
    #[test]
    fn t1(){
        let xml = r#"
    <?xml version = "1.0"?>
<SOAP-ENV:Envelope
   xmlns:SOAP-ENV = "http://www.w3.org/2001/12/soap-envelope"
   SOAP-ENV:encodingStyle = "http://www.w3.org/2001/12/soap-encoding">

   <SOAP-ENV:Body xmlns:m = "http://www.xyz.org/quotations">
      <m:GetQuotation>
         <m:QuotationsName>MiscroSoft</m:QuotationsName>
      </m:GetQuotation>
   </SOAP-ENV:Body>
</SOAP-ENV:Envelope>
    "#;
        println!("{}", match stj::STJ::xtj(xml) {
            Ok(v) => v,
            Err(_) => "err".into()
        });
    }
}

fn loopx(x: &mut Value) {
    if let Some(mp) = x.as_object_mut() {
        for (_, val) in mp {
            if let Some(obj) = val.as_object_mut() {
                if let Some(value) = obj.get("$value") {
                    *val = value.clone();
                } else {
                    loopx(val);
                }
            } else {
                loopx(val);
            }
        }
    } else if x.is_array() {
        for mut o in x.as_array().unwrap().clone().iter_mut() {
            loopx(o);
        }
    }
}


unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        size_of::<T>(),
    )
}
