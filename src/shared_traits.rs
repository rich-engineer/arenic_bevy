pub trait EnumDisplay {
    fn to_display_string(&self) -> String;
}

pub trait ComponentDisplay {
    fn to_display_string(&self) -> String;
}
