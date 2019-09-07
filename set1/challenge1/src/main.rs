use std::cmp;
use std::iter::Iterator;

fn main() {
    let inp = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let data = hex_to_bytes(inp);
    let b64 = base64::encode(&data);
    println!("{}", b64);

    assert_eq!(expected, b64);
}

fn hex_to_bytes(inp: &str) -> Vec<u8> {
    inp.slices(2)
        .map(|s| u8::from_str_radix(s, 16).expect("Bad input"))
        .collect()
}

struct Slices<'a> {
    inp: &'a str,
    slice_len: usize,
    cur: usize,
}

trait SlicesExt {
    fn slices(&self, len: usize) -> Slices;
}

impl SlicesExt for &str {
    fn slices(&self, slice_len: usize) -> Slices {
        Slices {
            inp: self,
            slice_len,
            cur: 0,
        }
    }
}

impl<'a> Iterator for Slices<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.cur < self.inp.len() {
            let cur = self.cur;
            self.cur = cmp::min(self.cur + self.slice_len, self.inp.len());
            Some(&self.inp[cur..self.cur])
        } else {
            None
        }
    }
}
