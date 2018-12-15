/*
Input: a vector of points (x, y)
output: a vector of lines (xbeg, ybeg, xend, yend)
*/
use std::cmp::Ordering;
use std::collections::HashSet;
//use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use line;
use point;
use event;


pub fn run(input:&Vec<(point::Point)>, output:&mut Vec<line::Line>) {
    
    let max_size:f64 = 10.0;
    //let mut voronoi_points:Vec<(point::Point)> = Vec::new();
    let mut events:BinaryHeap<(event::Event)> = BinaryHeap::new();
    let mut voronoi_vertices:Vec<Vec<usize> > = Vec::new();
    //let mut visited_points:HashSet<(usize, usize, usize)> = HashSet::new();
    let mut dirx:f64 = max_size;
    let mut tmp_pt:point::Point;
    for pt in input {
        //events.insert(event::Event::SiteEvent(*pt));
        tmp_pt = point::Point { _x: pt._x, _y: pt._y };
        events.push((event::Event::SiteEvent(tmp_pt)));
    }

    while (events.len() > 0) {
        // handle site and circle events
    }


    
}

