pub trait NodeTrait {
    fn new(id: u32, name: String, parent: Option<u32>, children: Option<(u32, u32)>) -> Self;
    fn get_id(&self) -> u32;
    fn get_name(&self) -> String;
    fn get_parent(&self) -> Option<u32>;
    fn get_children(&self) -> Option<(u32, u32)>;
}

pub struct Node {
    id: u32,
    name: String,
    parent: Option<u32>,
    children: Option<(u32, u32)>,
}

impl Node {
    pub(crate) fn new(
        id: u32,
        name: String,
        parent: Option<u32>,
        children: Option<(u32, u32)>,
    ) -> Node {
        Node {
            id,
            name,
            parent,
            children,
        }
    }
}
