mod json;

fn main() {
    let mut tokenizer = json::Tokenizer::new(std::io::stdin());

    loop {
        match tokenizer.next() {
            Some(Ok(tok)) => println!("token: {:?}", tok),
            Some(Err(err)) => {
                println!("error: {}", err);
                return;
            }
            None => break,
        }
    }

    println!("done")
}
