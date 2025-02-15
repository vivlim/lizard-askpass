use std::fmt::Debug;

#[derive(Debug)]
pub struct CharMatrix<T>
where
    T: Debug,
{
    pub layers: Vec<Layer<T>>,
}

#[derive(Debug, Default)]
pub struct Layer<T>
where
    T: Debug,
{
    pub rows: Vec<Row<T>>,
}

#[derive(Debug, Default)]
pub struct Row<T>
where
    T: Debug,
{
    pub items: Vec<T>,
}

#[derive(Debug, Default)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}
