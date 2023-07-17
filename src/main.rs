use lambda_mountain::{Rhs,Policy};

use openai_api_rust::*;
use openai_api_rust::chat::*;

fn random(args: &[Rhs]) -> Rhs {
   Rhs::Literal("Homer".to_string())
}

fn main() {
   let mut policy = Policy::new();
   policy.bind_extern("random", &random);

   let mut lm = std::process::Command::new("lm");
   for arg in std::env::args().skip(1) {
      lm.arg(arg);
   }
   let output = lm.output().expect("failed to execute lm process").stdout;
   let output = String::from_utf8_lossy(&output);
   let mut output = output.trim().to_string();
   if let Some((o,_)) = output.rsplit_once("\n") {
      output = o.to_string();
   }

   let auth = Auth::from_env().unwrap();
   let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
   let body = ChatBody {
       model: "gpt-3.5-turbo".to_string(),
       max_tokens: None,
       temperature: Some(0_f32),
       top_p: Some(0_f32),
       n: Some(2),
       stream: Some(false),
       stop: None,
       presence_penalty: None,
       frequency_penalty: None,
       logit_bias: None,
       user: None,
       messages: vec![Message { role: Role::User, content: output }],
   };
   let rs = openai.chat_completion_create(&body);
   let choice = rs.unwrap().choices;
   let message = &choice[0].message.as_ref().unwrap();
   println!("{}", message.content);
}
