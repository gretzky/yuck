use std::env;
use std::char;
use std::io::ErrorKind;
use std::collections::HashMap;

fn load_file(path: &str) -> Result<String, std::io::Error> {
  use std::io::Read;
  let mut data = String::new();
  try!(try!(std::fs::File::open(path)).read_to_string(&mut data));
  return Ok(data);
}

fn interpret(src: &str) -> Result<(), String> {
  let mut memory : [u32; 30000] = [0; 30000];
  let mut mem_pointer: usize = 0;
  let mut int_pointer: usize = 0;
  let mut bf_map = HashMap::new();

  use std::io;
  let mut in_buf = String::new();

  let mut bf_chars : Vec<usize> = Vec::new();
  let mut ii : usize = 0;
  loop {
    while !src.is_char_boundary(ii) { ii += 1; }
    if ii >= src.len() { break; }
    let c = src[ii..].chars().next().unwrap();
    match c {
      '[' => bf_chars.push(ii),
      ']' => {
        let p = try!(bf_chars.pop().ok_or(String::from("Too many closing brackets.")));
        bf_map.insert(p, ii);
        bf_map.insert(ii, p);
      },
      _ => (),
    }
    ii += 1;
  }
  if bf_chars.len() > 0 { return Err(String::from("Unmatched bracket")); }

  loop {
    if int_pointer >= src.len() { return Ok(()); }
    while !src.is_char_boundary(int_pointer) { int_pointer += 1; }
    let c = src[int_pointer..].chars().next().unwrap();
    match c {
      '>' => mem_pointer += 1,
      '<' => mem_pointer -= 1,
      '+' => memory[mem_pointer] += 1,
      '-' => memory[mem_pointer] -= 1,
      '.' => print!("{}", try!(char::from_u32(memory[mem_pointer])
                               .ok_or(format!("Invalid character: {}", memory[mem_pointer])))),
      ',' => {
        if in_buf.len() == 0 {
          try!(io::stdin().read_line(&mut in_buf).map_err(|e|format!("{}", e))) ;
        }
        memory[mem_pointer] = in_buf[0..1].chars().next().unwrap() as u32;
        in_buf = in_buf[1..].to_string();
      }


      '[' if memory[mem_pointer] == 0 => int_pointer = *bf_map.get(&int_pointer).unwrap(),
      ']' if memory[mem_pointer] != 0 => int_pointer = *bf_map.get(&int_pointer).unwrap(),
      _ => (),
    };
    int_pointer += 1;
  }
}


fn main() {
  let f = &String::from(env::args().skip(1).take(1).next().unwrap_or(String::from("")).trim());
  if f.is_empty() {
    println!("Usage: yuck <file>");
    return;
  }
  let src = load_file(f).map_err(|e| match e.kind() {
    ErrorKind::NotFound => String::from("File not found."),
    _ => format!("Unknown error: {}", e),
  });
  if src.is_err() {
    println!("{}", src.unwrap_err());
    return;
  }
  let res = interpret(&src.unwrap());
  if res.is_err() {
    println!("Error: {}", res.unwrap_err());
  }
}

