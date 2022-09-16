/* Parser for Algebraic Proofing System Language */

use std::fs;

#[path = "aps_parser/aps_parser.rs"] mod aps_parser;
#[path = "explorer/explorer.rs"] mod exporer;

fn main() {
    let input = match fs::read_to_string("test.aps") {
        Ok(content) => content,
        Err(err) => panic!("{}", err)
    };
    let operations = match aps_parser::root::<aps_parser::ApsParserKind>(&input) {
        Ok(("", value)) => value,
        Ok((rest, parsed)) => panic!(
            "unfinished parse: '{}'\nParsed content:\n{:#?}\n",
            rest,
            parsed
        ),
        Err(err) => panic!("{:#?}", err)
    };
    print!(
        "parsing result:\n{:#?}\n",
        operations
    )
}
