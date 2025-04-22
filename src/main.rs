use nexus::tokenizer::read_expected_token;

fn main() {
    let binding = String::from("text plus some other");
    let iter = binding.chars();
    
    let iter = read_expected_token(iter, String::from("text")).expect("msg");

    dbg!(iter.collect::<Vec<_>>());
}
