#[derive(Clone, Debug)]
pub struct StrVal(String);

impl Into<Box<str>> for StrVal {
    fn into(self) -> Box<str> {
        self.0.into()
    }
}