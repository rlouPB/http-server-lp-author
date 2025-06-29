#[derive(Debug, PartialEq)]
enum Lang {
  English,
  Spanish,
  Chinese,
  Texan,
  French,
  German,
  Italian,
  Portuguese,
  Russian,
}
 
struct Greeting {
    message: String,
    lang: Lang,
}

fn get_greeting_message(greetings: &Vec<Greeting>, lang: &Lang) -> Option<String> {
    for e in greetings {
        if e.lang == *lang {
            return Some(e.message.clone());
        }
    }
    None
}
fn main() {
  let mut v :Vec<Greeting> = Vec::new();

  let g : Greeting = Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Texan, message: String::from("Howdy WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") };
  v.push(g);

  let g : Greeting = Greeting { lang: Lang::French, message: String::from("Bonjour WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::German, message: String::from("Hallo WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Italian, message: String::from("Ciao WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Portuguese, message: String::from("Olá WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Russian, message: String::from("Привет (Privet) WasmEdge!") };
  v.push(g);

  // for e in v {
  //   println!("{:?} {}", e.lang, e.message);
  // }

  // for e in v {
  //   if e.lang == Lang::Italian {
  //     println!("{:?} {}", e.lang, e.message);
  //     break;
  //   }
  // }
  let lang = Lang::Italian;
  let message = get_greeting_message(&v, &lang);
  println!("{:?} {}", lang, message.unwrap());
}
