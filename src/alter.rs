#[derive(Debug)]
pub enum Alter {
    Lowered,
    Natural,
    Sharp,
}
impl From<Option<String>> for Alter {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => {
                let n = s.parse::<i8>().unwrap();
                match n {
                    1 => Alter::Sharp,
                    -1 => Alter::Lowered,
                    _ => unreachable!(),
                }
            }
            None => Alter::Natural,
        }
    }
}
