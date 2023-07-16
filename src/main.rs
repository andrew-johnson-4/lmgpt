use lambda_mountain::Policy;

fn main() {
   let mut policy = Policy::new();
   let mut lm = std::process::Command::new("lm");
   for arg in std::env::args().skip(1) {
      lm.arg(arg);
   }
   let output = lm.output().expect("failed to execute lm process").stdout;
   let output = String::from_utf8_lossy(&output);
   let output = output.trim();
   if let Some((output,_)) = output.rsplit_once("\n") {
      println!("{}", output);
   }
}
