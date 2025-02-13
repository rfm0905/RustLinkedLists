// A list is either empty or an element followed by a list
pub enum List {
    Empty,
    Elem(i32, Box<List>)  // use box for heap allocation, since list is recursive and can be unknown size
}

