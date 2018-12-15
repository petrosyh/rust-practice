use point::Point;
use std::fmt;

// it needs AVL Tree for arc...

// Arc for Circle Event.
pub struct Arc {
    pub site: Point, // site point of this arc
}

pub struct BreakPt {
    pub left_arc_id: usize, //Point,
    pub right_arc_id: usize, //Point,
}

pub enum NodeData {
    ArcData(Arc),
    BreakPtData(BreakPt)
}

// Tree Node for Arc and break pt
pub struct ArcNode {
    pub id: usize, //i32,
    pub parent: Option<Box<ArcNode>>,
    pub lchild: Option<Box<ArcNode>>,
    pub rchild: Option<Box<ArcNode>>,
    pub data: NodeData,
}

// Binary Tree for Arc and break pt
pub struct ArcTree {
    pub nodeSet: Vec<ArcNode>,
    pub root: Option<ArcNode>,
}


// BeachLine
pub struct ArcTriple {
    // arcs end points must same.
    pub left_arc: Arc,
    pub left_breakpt: Point,
    pub ceter_arc: Arc,
    pub right_breakpt: Point,
    pub right_arc: Arc,
    pub event_point: Point, // point for Circle Event
}

impl fmt::Debug for ArcTriple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({0:.1}, {1:.1})", self.event_point._x, self.event_point._y)
    }
}

impl ArcTree {
    pub fn new() -> Self {
        ArcTree { nodeSet : vec![], root : None }
    }

    pub fn add_root(&mut self, pt: Point) {
        let x = pt._x;
        let y = pt._y;
        let mut new_root_data = NodeData::ArcData(Arc{ site: pt });
        let mut new_root = ArcNode{ id: 0, parent: None, lchild: None, rchild: None, data: new_root_data };
        self.nodeSet.push(new_root);
        self.root = Some(new_root);
        //self.nodeSet.push(ArcNode{ id: 0, parent: None, lchild: None, rchild: None, data: NodeData::ArcData(Arc{ site: Point { _x:x, _y:y } }) });
        //self.root = Some(ArcNode{ id: 0, parent: None, lchild: None, rchild: None, data: NodeData::ArcData(Arc{ site: Point { _x:x, _y:y } }) });
    }

    pub fn find_arc_above(&self, pt: Point) -> ArcNode {
        let cur = &self.root.unwrap();

        loop {
            match &self.nodeSet[cur.id].data {
                NodeData::ArcData(Arc) => { return *cur; }
                NodeData::BreakPtData(BreakPt) => { 
                    //if (&self.nodeSet[BreakPt.left_arc_id] == pt.id || )
                    return *cur;
                }
            }
        }

    }
    



}
