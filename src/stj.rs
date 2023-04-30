use serde_json::Value;

pub struct STJ;


// converts xml to json, and values like {"foo": {"$value": "bar"}} is converted to {"foo": "bar"}

impl STJ{
    pub fn xtj(x: &str) -> crate::Result<Value>{
        let mut x:Value = serde_xml_rs::from_str(x)?;
        loopx(&mut x);
        Ok(x)
    }

    pub fn xtj_fromval(x: Value) -> crate::Result<Value> {
        let mut x = x;
        loopx(&mut x);
        Ok(x)
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
