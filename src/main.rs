macro_rules! error {
  ($($arg:tt)*) => {{
    eprintln!($($arg)*);
    std::process::exit(1);
  }};
}

fn main() {
  let args: Vec<_> = std::env::args().collect();

  match args.len() {
    1 => error!("Missing line numbers"),
    _ => {
      let line_numbers = args
        .iter()
        .skip(1)
        .map(|arg| match arg.parse::<usize>() {
          Ok(line) if line < 1 => {
            error!("All arguments must be positive")
          }
          Ok(line) => line - 1,
          Err(_) => error!("All arguments must be numbers"),
        })
        .collect::<Vec<_>>();

      let mut lines = vec![];

      loop {
        let mut buffer = String::new();
        match std::io::stdin().read_line(&mut buffer) {
          Ok(bytes) if bytes == 0 => break,
          Ok(_) => lines.push(buffer.trim().to_string()),
          Err(_) => error!("Failed to read from stdin"),
        }
      }

      for line_number in line_numbers {
        match lines.get(line_number) {
          Some(line) => println!("{line}"),
          None => {
            error!("There isn't a line {line_number}")
          }
        }
      }
    }
  }
}
