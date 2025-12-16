mod token;
mod parser;

fn main() {
    let p = parser::Parser::new();

    let nodes = p.parser("dsfhdgkdghdukghdfuk");

    println!("{:?} {:?}", p, nodes);
}
