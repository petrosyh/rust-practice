use point::Point;
use std::fmt;

// it needs AVL Tree for arc...

// Arc for Circle Event.
pub struct Arc{
    pub site: Point, // site point of this arc
}

// a Node for BeachLine
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
