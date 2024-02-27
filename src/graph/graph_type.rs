
// Perhaps there is a better way to do this
// But const ADTs are still experimental
pub trait GraphType {
    const SELF_LOOPS: bool;
    const MULTI_EDGES: bool;
}

pub struct Simple;
impl GraphType for Simple {
    const SELF_LOOPS: bool = false;
    const MULTI_EDGES: bool = false;
}
pub struct Multigraph;

impl GraphType for Multigraph {
    const SELF_LOOPS: bool = true;
    const MULTI_EDGES: bool = true;
}

// For type erased graphs
// Type erasure prevents nasty generics from
// precipitating down to the caller
pub struct Erased;

impl GraphType for Erased {
    const SELF_LOOPS: bool = true;
    const MULTI_EDGES: bool = true;
}