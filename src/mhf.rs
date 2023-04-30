pub struct MHF;

// MHF stands for MyHashFunction, designed by https://github.com/ssddOnTop and is put under MIT license. u128 can byte length < 347 so it isn't used in this project.
// we planned to use it to cache requests and improve performance but because of possible large size of xml we went for Cryptographic algo instead (sha256)

impl MHF{
    pub fn hsh(a: &[u8]) -> u128{
        let mut flag = 0;
        let mut gintx;
        let mut ix;
        let len = a.len();
        for i in 0..len {
            ix = a[i] as u128;
            gintx = gint(ix);
            flag = flag ^ gintx ^ ix;
            flag = cls(flag, gintx & 126);
        }
        flag
    }
    pub fn fnv1(key: &[u8]) -> usize {
        let mut hash = 0x811c9dc5;
        let prime = 0x01000193;
        for i in key {
            hash = (hash ^ &(*i as usize));
            hash = ((hash) * &prime);
        }
        hash
    }
}

fn gint(no: u128) -> u128 {
    (1 << (no & 127)) + (no >> 7)
}
fn cls(n: u128, d: u128) -> u128 {
    (n << d) | (n >> (127 - d))
}
