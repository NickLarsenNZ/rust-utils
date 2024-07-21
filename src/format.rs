pub trait DebugExt: std::fmt::Debug {
    fn format_debug(&self) -> String {
        format!("{self:?}")
    }
}

pub trait DisplayExt: std::fmt::Display {
    fn format_display(&self) -> String {
        format!("{self}")
    }
}

impl<T> DebugExt for T where T: std::fmt::Debug {}
impl<T> DisplayExt for T where T: std::fmt::Display {}
