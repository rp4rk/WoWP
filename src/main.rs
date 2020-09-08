mod parser;

use parser::cells::parse_log_line;

fn main() {
    let test_string = r#"EMOTE,Creature-0-3781-2164-22090-152853-000027F1FF,"Silivaz the Zealous",0000000000000000,nil,|TInterface\ICONS\SPELL_NATURE_EARTHQUAKE.BLP:20|t Silivaz is casting |cFFFF0000|Hspell:301807|h[Zealous Eruption]|h|r!"#;

    let res = parse_log_line(test_string);
    println!("{:?}", res);
}
