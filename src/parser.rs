use ast;

lalrpop_mod!(pub eer_grammar);

pub fn convert(src: impl AsRef<str>) -> ast::EER {
    let parser = eer_grammar::EERParser::new();
    parser.parse(src.as_ref()).unwrap()
}
