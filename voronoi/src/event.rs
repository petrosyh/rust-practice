use point::Point;
//use arc::Arc;
use arc::ArcTriple;
use std::cmp::Ordering;

#[derive(Debug)]
pub enum Event {
    SiteEvent(Point),
    CirEvent(Point, f64, ArcTriple)
}


impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_height().partial_cmp(&(other.get_height()))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.get_height() == other.get_height()
    }
}

impl Eq for Event { }

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Greater)
    }

}

impl Event {
    pub fn get_height(&self) -> f64 {
        match self {
            Event::SiteEvent(ref pt) => pt._y,
            Event::CirEvent(pt, radius, ArcTriple) => pt._y + radius,
        }
    }
}