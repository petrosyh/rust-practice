use std::fmt;

pub struct Point {
    pub _x: f64,
    pub _y: f64,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({0:.1}, {1:.1})", self._x, self._y)
    }
}


//pub struct SiteEvent {
 //   pub p: Point,
//}

//pub struct CirEvent {
//    pub pt: Point,
 //   pub para: Parabo,
 //   pub y: f64,
//}

//pub struct Parabo {
 //   pub pt: Point,
//    pub ev: CirEvent,
//}
