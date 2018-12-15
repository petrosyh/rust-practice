/*
Input: a vector of points (x, y)
output: a vector of lines (xbeg, ybeg, xend, yend)
*/
use std::cmp::Ordering;
//use std::collections::HashSet;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use line;
use point::Point;
use event;


pub fn run(input:&Vec<(Point)>, output:&mut Vec<line::Line>) {
    
    let max_size:f64 = 10.0;
    //let mut voronoi_points:Vec<(point::Point)> = Vec::new();
    let mut events:BTreeSet<(event::Event)> = BTreeSet::new();
    let mut voronoi_vertices:Vec<Vec<usize> > = Vec::new();

    //let mut visited_points:HashSet<(usize, usize, usize)> = HashSet::new();
    let mut dirx:f64 = max_size;
    let mut tmp_pt:Point;



    for pt in input {
        //events.insert(event::Event::SiteEvent(*pt));
        tmp_pt = Point { _x: pt._x, _y: pt._y };
        events.insert((event::Event::SiteEvent(tmp_pt)));
    }

    while (!events.is_empty()) {
        // handle site and circle events

    }


    
}

//fn site_event_handler(pt: Point, events: &mut BTreeSet<(event::Event)>, )

