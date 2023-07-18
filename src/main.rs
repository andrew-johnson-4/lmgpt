use lambda_mountain::{Rhs,Policy};

use openai_api_rust::*;
use openai_api_rust::chat::*;

use std::fs::File;
use std::io::Read;

fn chat(prompt: &str) -> String {
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
       messages: vec![Message { role: Role::User, content: prompt.to_string() }],
   };
   let rs = openai.chat_completion_create(&body);
   let choice = rs.unwrap().choices;
   let message = &choice[0].message.as_ref().unwrap();
   return message.content.clone();
}

fn cat(args: &[Rhs]) -> String {
   let mut s = String::new();
   for (i,a) in args.iter().enumerate() {
      if i>0 {
         s.push(' ');
      }
      s.push_str(&a.to_string());
   }
   s
}

fn hashtags(args: &[Rhs]) -> Rhs {
   let subject = cat(args);
   Rhs::Literal(chat(&format!("Suggest a list of hashtags to describe this document: {}",subject)))
}

fn keywords(args: &[Rhs]) -> Rhs {
   let subject = cat(args);
   Rhs::Literal(chat(&format!("Suggest a list of keywords to describe this document: {}",subject)))
}

fn random(args: &[Rhs]) -> Rhs {
   let subject = cat(args);
   Rhs::Literal(chat(&format!("Tell me the name of a random {}.",subject)))
}

fn autocomplete(args: &[Rhs]) -> Rhs {
   let subject = cat(args);
   Rhs::Literal(chat(&format!("Complete the following sentence. {}",subject)))
}

fn translate(args: &[Rhs]) -> Rhs {
   let mut lang = "English".to_string();
   if args.len()>0 {
      lang = args[0].to_string();
   }
   let subject = cat(&args[1..]);
   Rhs::Literal(chat(&format!("Translate the following text into {}: {}.",lang,subject)))
}

fn main() {
   let mut policy = Policy::new();
   policy.bind_extern("random", &random);
   policy.bind_extern("translate", &translate);
   policy.bind_extern("hashtags", &hashtags);
   policy.bind_extern("keywords", &keywords);
   policy.bind_extern("autocomplete", &autocomplete);
   policy.load(include_str!("../preludes/lmgpt.lm")).expect("unable to read prelude lmgpt.lm");

   let mut prompt = String::new();
   for arg in std::env::args().skip(1) {
      let mut p = String::new();
      let mut file = File::open(arg).expect("load_policy: error opening file");
      file.read_to_string(&mut p).expect("load_policy: unable to read to string");
      if let Result::Err(o) = policy.load(&p) {
         prompt = o.clone();
      }
   }

   if prompt.len()>0 {
      println!("{}", chat(&prompt));
   }
}
