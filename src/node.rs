#[derive(Debug)]
pub enum Node {
    Number(i32),
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Wait(i32),
    Sequence(Vec<Node>)
}