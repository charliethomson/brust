extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .emit_rerun_directives(true)
        .always_use_colors()
        .process()
        .unwrap();
}
