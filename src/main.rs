use lalrpop_util::lexer::Token;

#[macro_use]
extern crate lalrpop_util;
pub mod ast;
mod grammar;
pub mod interpreter;

pub type Parser = grammar::FileParser;
fn main() {
    let s = r#"
    main() {
        while (i--) {
            j = f(i);
            switch (j) {
                case 1:
                    x = 5;
                    break;
                case 2:
                case 3:
                    x = g(j);
            }
        labl:
            if( j < 0 ) {
                xy = 3;
                break;
            }
            xy = j/2;
        }
        return xy;
    }
    "#;
    println!("{:#?}", Parser::new().parse(s));
}
