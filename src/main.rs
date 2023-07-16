use lambda_mountain::Policy;

fn main() {
   let mut policy = Policy::new();
   for arg in std::env::args().skip(1) {
      println!("{}", arg);
   }
}
