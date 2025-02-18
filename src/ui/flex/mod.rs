pub mod flex1;
pub mod flex2;
pub mod flex_row;
pub mod flex_row_test;

#[derive(Debug, Clone, Copy)]
pub enum FlexAxisAlign {
    Start,
    End,
    Between,
    Stretch,
}
