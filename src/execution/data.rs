
#[derive(Debug, Clone, Copy)]
pub struct HeapAddress(pub usize);

#[derive(Debug)]
pub enum Data {
    Bool(bool),
    Number(i64),
    String(String),
    Ref(HeapAddress),
}