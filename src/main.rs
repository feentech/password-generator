use rand::{thread_rng, Rng};

const CHARSET: &[u8] = br#"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789~`!@#$%^&*()-=_+[]\{}|;':",./<>?"#;
fn main() {
  let arg = std::env::args().nth(1).unwrap_or("16".to_string());
  let length = match arg.parse::<u16>() {
    Ok(length) => length,
    Err(e) => return eprintln!("{}", e.to_string()),
  };

  let mut rng = thread_rng();
  let password = (0..length)
    .map(|_| {
      let i = rng.gen_range(0..CHARSET.len());
      CHARSET[i] as char
    })
    .collect::<String>();

  println!("{}", password);
}
