pub enum BehaviorTreeType {
    NormalEnemy,
    AggressiveEnemy,
    ElusiveEnemy,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Behavior {
    Idle,
    MoveToPlayer,
    MoveToRandom,
    AttackPlayer,
    AttackRandom,
    RunAway,
    Dodge,
}

pub struct EnemyBehaviors {
    pub(crate) root: Node,
    pub(crate) nodes: Vec<Node>,
}

pub trait NodeTrait {
    fn new(id: u32, name: Behavior, parent: Option<u32>, children: Option<(u32, u32)>) -> Self;
    fn get_id(&self) -> u32;
    fn get_name(&self) -> Behavior;
    fn get_parent(&self) -> Option<u32>;
    fn get_children(&self) -> Option<(u32, u32)>;
}

#[derive(Debug, Clone)]
pub struct Node {
    id: u32,
    name: Behavior,
    parent: Option<u32>,
    children: Option<(u32, u32)>,
}

impl Node {
    pub(crate) fn new(
        id: u32,
        name: Behavior,
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

impl NodeTrait for Node {
    fn new(id: u32, name: Behavior, parent: Option<u32>, children: Option<(u32, u32)>) -> Node {
        Node {
            id,
            name,
            parent,
            children,
        }
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_name(&self) -> Behavior {
        self.name.clone()
    }

    fn get_parent(&self) -> Option<u32> {
        self.parent
    }

    fn get_children(&self) -> Option<(u32, u32)> {
        self.children
    }
}

pub struct BehaviorTree {
    root: Node,
    nodes: Vec<Node>,
    current_node: u32,
}

impl BehaviorTree {
    pub(crate) fn new(root: Node, nodes: Vec<Node>) -> BehaviorTree {
        BehaviorTree {
            root,
            nodes,
            current_node: 0,
        }
    }
}

pub trait BehaviorTreeTrait {
    fn new() -> Self
    where
        Self: Sized;
    fn get_root(&self) -> &Node;
    fn get_next_behaviors(&self, node_id: u32) -> Option<(u32, u32)>;
    fn get_node(&self, node_id: u32) -> Option<&Node>;
    fn get_node_name(&self, node_id: u32) -> Option<Behavior>;
    fn get_node_parent(&self, node_id: u32) -> Option<u32>;
    fn get_node_children(&self, node_id: u32) -> Option<(u32, u32)>;
    fn get_current_node(&self) -> u32;
}
pub trait CustomBehaviorTreeTrait {
    fn from(behavior_tree: BehaviorTree) -> Self;
    fn get_root(&self) -> &Node;
    fn get_next_behaviors(&self, node_id: u32) -> Option<(u32, u32)>;
    fn get_node(&self, node_id: u32) -> Option<&Node>;
    fn get_node_name(&self, node_id: u32) -> Option<Behavior>;
    fn get_node_parent(&self, node_id: u32) -> Option<u32>;
    fn get_node_children(&self, node_id: u32) -> Option<(u32, u32)>;
    fn get_current_node(&self) -> u32;
}
