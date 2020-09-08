mod parser;

use parser::parse_log_line;

fn main() {
    let test_string = r#"10/17 00:56:59.121  SPELL_AURA_REFRESH,Player-3661-07DAB733,"Frozenpriest-Hyjal",0x514,0x0,Player-3661-07DAB733,"Frozenpriest-Hyjal",0x514,0x0,123254,"Twist of Fate",0x1,BUFF"#;

    let res = parse_log_line(test_string);
    println!("{:?}", res);
}
