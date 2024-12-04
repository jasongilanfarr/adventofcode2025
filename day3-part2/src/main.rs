use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"] 
struct MulParser;

fn main() -> std::io::Result<()> {
    let stdin = std::io::read_to_string(std::io::stdin())?;
    let mut sum = 0;
    let mut should_count = true;
    let full = MulParser::parse(Rule::grammar, &stdin)
    .expect("unsuccessful parse")
    .next()
    .unwrap();

    for chunk in full.into_inner() {
        match chunk.as_rule() {
            Rule::mul => {
                let mut inner_rules = chunk.into_inner();
                let lhs = inner_rules.next().unwrap().as_str().parse::<i32>().unwrap();
                let rhs = inner_rules.next().unwrap().as_str().parse::<i32>().unwrap();
                if should_count {
                    sum += lhs * rhs;
                }
            },
            Rule::doit => {
                should_count = true;
            },
            Rule::dont => {
                should_count = false;
            },
            _ => (),
        }
    }

    println!("{}", sum);

    Ok(())
}
