use std::fs::File;
use std::path::PathBuf;
use std::io::{prelude::*, BufReader};

/// Signature Known Answer Tests
#[derive(Debug, Clone)]
pub struct Kat {
  pub seed: Vec<u8>,
  pub mlen: usize,
  pub msg: Vec<u8>,
  pub pk: Vec<u8>,
  pub sk: Vec<u8>,
  pub smlen: usize,
  pub sm: Vec<u8>,
}

/// Converts string octuples from .rsp files into Kat structs
impl From<&[String]> for Kat {
  fn from(kat: &[String]) -> Self {
    let values: Vec<String> = kat.iter()
      .map(
        |katline| {
          let val: Vec<&str> = katline.split("= ").collect();
          // Handle blank lines
          if val.len() > 1 { val[1].into() } else { val[0].into() }
        }
      ).collect();

    // Ignore count at index 0
    Kat {
      seed: decode_hex(&values[1].clone()),
      mlen: values[2].parse::<usize>().unwrap(),
      msg: decode_hex(&values[3].clone()),
      pk: decode_hex(&values[4].clone()),
      sk: decode_hex(&values[5].clone()),
      smlen: values[6].parse::<usize>().unwrap(),
      sm: decode_hex(&values[7].clone()),
    }
  }
}

/// Packs chunks of lines into signature Kat structs 
pub fn kats(path: &mut PathBuf, filename: &str) -> Vec<Kat> {
  path.extend(&["tests", "KAT", filename]);
  dbg!(&path);
  let file = File::open(path).expect("Error loading KAT file");
  let buf = BufReader::new(file);
  let lines: Vec<String>  = buf.lines()
    .map(|l| l.expect("Unable to parse KAT line"))
    .collect();
  
  let kats = lines[2..].chunks_exact(9);  // Skip first 2 lines
  kats.map( |c| {c.into()} ).collect::<Vec<Kat>>()
}

/// Loads the deterministic seed buffers from file.
/// Buffers are newline separated. Path is the crate root 
pub fn bufs(path: &mut PathBuf, filename: &str) -> Vec<Vec<u8>> {
  path.extend(&["tests", "KAT", filename]);
  let file = File::open(path).expect("Error loading buf file");
  let buf = BufReader::new(file);
  buf.lines()
    .map(|l| decode_hex(&l.unwrap()))
    .collect()
}


/// Decodes hex string into a vector of bytes
fn decode_hex(s: &str) -> Vec<u8> {
  (0..s.len())
    .step_by(2)
    .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("Hex string decoding"))
    .collect::<Vec<u8>>()
}

