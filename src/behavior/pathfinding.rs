use crate::objects::Point;
use crate::state::Context;
use std::collections::HashMap;

pub type Weight = f32;
pub type Score = f32;
pub type Cost = f32;

pub struct Pathfinder {
    nodes: HashMap<Point, Cost>,
    start: Point,
    end: Point,
}

impl Pathfinder {
    pub fn new(nodes: HashMap<Point, Cost>, start: Point, end: Point) -> Self {
        Self { nodes, start, end }
    }

    pub fn limit(&self) -> u32 {
        let w = Context::width();
        let h = Context::height();
        w * h
    }

    pub fn find(&self) -> Vec<Point> {
        let max = self.limit();

        let weights: HashMap<Point, (Cost, Weight)> = self
            .nodes
            .iter()
            .map(|(p, c)| (*p, (*c, p.distance(self.end) as f32)))
            .collect();

        let (_, w): (Cost, Weight) = weights
            .get(&self.start)
            .expect("Starting point is not in map")
            .to_owned();

        let mut scores: HashMap<Point, Score> = HashMap::new();

        // starting node score is just weight because we're
        // already there, so no cost.
        scores.insert(self.start, w);

        let mut queue = vec![(self.start, w, Vec::<Point>::new(), 0.0)];

        let mut count = 0;
        while queue[0].0 != self.end {
            let (point, _, path, previous) = queue.swap_remove(0);
            for node in point.neighbors().iter() {
                let (current, _) = weights.get(&point).expect("Cannot find current weights");
                let (next, weight) = weights.get(&node).expect("Cannot find next weights");

                let cost = previous + (current * 0.5) + (next * 0.5);
                let score = weight + cost;

                let mut traversed = path.clone();
                traversed.push(point);

                if scores.contains_key(node) {
                    if scores.get(node) >= Some(&score) {
                        scores.insert(*node, score);

                        // find an existing queued item with a score better than the current
                        // path and update it, otherwise insert a new queue item.
                        match queue
                            .iter_mut()
                            .find(|(p, w, _, _)| p == node && w >= &score)
                        {
                            Some(q) => {
                                q.1 = score;
                                q.2 = traversed;
                                q.3 = cost;
                            }
                            None => {
                                queue.push((*node, score, traversed, cost));
                            }
                        };
                    }
                } else {
                    scores.insert(*node, score);
                    queue.push((*node, score, traversed, cost));
                }
            }

            queue.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            // break when no path found
            if count > max {
                break;
            }
            count += 1;
        }

        let mut best = queue[0].2.clone();
        best.push(self.end);
        best
    }

    pub fn find_weighted(&self) -> Vec<(Point, Cost)> {
        self.find()
            .into_iter()
            .map(|p| (p, self.nodes.get(&p).unwrap().clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! initialize {
        ( $w:expr, $h:expr ) => {
            initialize!($w, $h, 175, 200);
        };
        ( $w:expr, $h:expr, $tw:expr, $th:expr ) => {
            crate::state::Context::set_size($w, $h);
            crate::state::Context::set_tile_size($tw, $th);
        };
    }

    macro_rules! map {
        ($( $t: expr),*) => {{
             let mut map = HashMap::new();
             $( map.insert($t.0, $t.1); )*
             map
        }}
    }

    #[test]
    fn pathfinder_simple_test() {
        initialize!(30, 30);

        let tilemap = map![
            (Point::new(-15, -15), 8.),
            (Point::new(-14, -15), 4.),
            (Point::new(-13, -15), 5.),
            (Point::new(-12, -15), 11.),
            (Point::new(-11, -15), 13.),
            (Point::new(-10, -15), 16.),
            (Point::new(-9, -15), 23.),
            (Point::new(-8, -15), 31.),
            (Point::new(-7, -15), 31.),
            (Point::new(-6, -15), 28.),
            (Point::new(-5, -15), 23.),
            (Point::new(-4, -15), 14.),
            (Point::new(-3, -15), 8.),
            (Point::new(-2, -15), 5.),
            (Point::new(-1, -15), 5.),
            (Point::new(0, -15), 4.),
            (Point::new(1, -15), 3.),
            (Point::new(2, -15), 3.),
            (Point::new(3, -15), 3.),
            (Point::new(4, -15), 3.),
            (Point::new(5, -15), 3.),
            (Point::new(6, -15), 3.),
            (Point::new(7, -15), 3.),
            (Point::new(8, -15), 3.),
            (Point::new(9, -15), 4.),
            (Point::new(10, -15), 6.),
            (Point::new(11, -15), 8.),
            (Point::new(12, -15), 8.),
            (Point::new(13, -15), 7.),
            (Point::new(14, -15), 8.),
            (Point::new(-15, -14), 10.),
            (Point::new(-14, -14), 5.),
            (Point::new(-13, -14), 5.),
            (Point::new(-12, -14), 12.),
            (Point::new(-11, -14), 17.),
            (Point::new(-10, -14), 21.),
            (Point::new(-9, -14), 28.),
            (Point::new(-8, -14), 31.),
            (Point::new(-7, -14), 31.),
            (Point::new(-6, -14), 25.),
            (Point::new(-5, -14), 17.),
            (Point::new(-4, -14), 9.),
            (Point::new(-3, -14), 4.),
            (Point::new(-2, -14), 3.),
            (Point::new(-1, -14), 4.),
            (Point::new(0, -14), 4.),
            (Point::new(1, -14), 3.),
            (Point::new(2, -14), 3.),
            (Point::new(3, -14), 4.),
            (Point::new(4, -14), 5.),
            (Point::new(5, -14), 6.),
            (Point::new(6, -14), 4.),
            (Point::new(7, -14), 3.),
            (Point::new(8, -14), 3.),
            (Point::new(9, -14), 4.),
            (Point::new(10, -14), 6.),
            (Point::new(11, -14), 8.),
            (Point::new(12, -14), 7.),
            (Point::new(13, -14), 7.),
            (Point::new(14, -14), 8.),
            (Point::new(-15, -13), 12.),
            (Point::new(-14, -13), 6.),
            (Point::new(-13, -13), 4.),
            (Point::new(-12, -13), 9.),
            (Point::new(-11, -13), 17.),
            (Point::new(-10, -13), 21.),
            (Point::new(-9, -13), 23.),
            (Point::new(-8, -13), 23.),
            (Point::new(-7, -13), 23.),
            (Point::new(-6, -13), 19.),
            (Point::new(-5, -13), 13.),
            (Point::new(-4, -13), 7.),
            (Point::new(-3, -13), 3.),
            (Point::new(-2, -13), 4.),
            (Point::new(-1, -13), 3.),
            (Point::new(0, -13), 3.),
            (Point::new(1, -13), 3.),
            (Point::new(2, -13), 3.),
            (Point::new(3, -13), 4.),
            (Point::new(4, -13), 8.),
            (Point::new(5, -13), 8.),
            (Point::new(6, -13), 6.),
            (Point::new(7, -13), 4.),
            (Point::new(8, -13), 3.),
            (Point::new(9, -13), 4.),
            (Point::new(10, -13), 7.),
            (Point::new(11, -13), 9.),
            (Point::new(12, -13), 9.),
            (Point::new(13, -13), 9.),
            (Point::new(14, -13), 9.),
            (Point::new(-15, -12), 15.),
            (Point::new(-14, -12), 8.),
            (Point::new(-13, -12), 3.),
            (Point::new(-12, -12), 6.),
            (Point::new(-11, -12), 13.),
            (Point::new(-10, -12), 17.),
            (Point::new(-9, -12), 17.),
            (Point::new(-8, -12), 17.),
            (Point::new(-7, -12), 17.),
            (Point::new(-6, -12), 17.),
            (Point::new(-5, -12), 13.),
            (Point::new(-4, -12), 7.),
            (Point::new(-3, -12), 3.),
            (Point::new(-2, -12), 4.),
            (Point::new(-1, -12), 4.),
            (Point::new(0, -12), 3.),
            (Point::new(1, -12), 3.),
            (Point::new(2, -12), 3.),
            (Point::new(3, -12), 4.),
            (Point::new(4, -12), 7.),
            (Point::new(5, -12), 8.),
            (Point::new(6, -12), 5.),
            (Point::new(7, -12), 3.),
            (Point::new(8, -12), 3.),
            (Point::new(9, -12), 5.),
            (Point::new(10, -12), 9.),
            (Point::new(11, -12), 11.),
            (Point::new(12, -12), 12.),
            (Point::new(13, -12), 12.),
            (Point::new(14, -12), 13.),
            (Point::new(-15, -11), 14.),
            (Point::new(-14, -11), 10.),
            (Point::new(-13, -11), 6.),
            (Point::new(-12, -11), 4.),
            (Point::new(-11, -11), 9.),
            (Point::new(-10, -11), 16.),
            (Point::new(-9, -11), 16.),
            (Point::new(-8, -11), 16.),
            (Point::new(-7, -11), 14.),
            (Point::new(-6, -11), 14.),
            (Point::new(-5, -11), 12.),
            (Point::new(-4, -11), 7.),
            (Point::new(-3, -11), 3.),
            (Point::new(-2, -11), 5.),
            (Point::new(-1, -11), 5.),
            (Point::new(0, -11), 4.),
            (Point::new(1, -11), 3.),
            (Point::new(2, -11), 3.),
            (Point::new(3, -11), 5.),
            (Point::new(4, -11), 7.),
            (Point::new(5, -11), 8.),
            (Point::new(6, -11), 4.),
            (Point::new(7, -11), 3.),
            (Point::new(8, -11), 5.),
            (Point::new(9, -11), 5.),
            (Point::new(10, -11), 7.),
            (Point::new(11, -11), 9.),
            (Point::new(12, -11), 12.),
            (Point::new(13, -11), 14.),
            (Point::new(14, -11), 16.),
            (Point::new(-15, -10), 12.),
            (Point::new(-14, -10), 10.),
            (Point::new(-13, -10), 7.),
            (Point::new(-12, -10), 3.),
            (Point::new(-11, -10), 7.),
            (Point::new(-10, -10), 12.),
            (Point::new(-9, -10), 13.),
            (Point::new(-8, -10), 12.),
            (Point::new(-7, -10), 11.),
            (Point::new(-6, -10), 9.),
            (Point::new(-5, -10), 8.),
            (Point::new(-4, -10), 5.),
            (Point::new(-3, -10), 3.),
            (Point::new(-2, -10), 4.),
            (Point::new(-1, -10), 6.),
            (Point::new(0, -10), 5.),
            (Point::new(1, -10), 4.),
            (Point::new(2, -10), 4.),
            (Point::new(3, -10), 6.),
            (Point::new(4, -10), 8.),
            (Point::new(5, -10), 8.),
            (Point::new(6, -10), 4.),
            (Point::new(7, -10), 4.),
            (Point::new(8, -10), 5.),
            (Point::new(9, -10), 5.),
            (Point::new(10, -10), 5.),
            (Point::new(11, -10), 7.),
            (Point::new(12, -10), 9.),
            (Point::new(13, -10), 13.),
            (Point::new(14, -10), 17.),
            (Point::new(-15, -9), 9.),
            (Point::new(-14, -9), 9.),
            (Point::new(-13, -9), 8.),
            (Point::new(-12, -9), 4.),
            (Point::new(-11, -9), 4.),
            (Point::new(-10, -9), 7.),
            (Point::new(-9, -9), 7.),
            (Point::new(-8, -9), 7.),
            (Point::new(-7, -9), 8.),
            (Point::new(-6, -9), 8.),
            (Point::new(-5, -9), 7.),
            (Point::new(-4, -9), 5.),
            (Point::new(-3, -9), 5.),
            (Point::new(-2, -9), 3.),
            (Point::new(-1, -9), 5.),
            (Point::new(0, -9), 6.),
            (Point::new(1, -9), 6.),
            (Point::new(2, -9), 8.),
            (Point::new(3, -9), 10.),
            (Point::new(4, -9), 12.),
            (Point::new(5, -9), 10.),
            (Point::new(6, -9), 5.),
            (Point::new(7, -9), 4.),
            (Point::new(8, -9), 5.),
            (Point::new(9, -9), 5.),
            (Point::new(10, -9), 4.),
            (Point::new(11, -9), 5.),
            (Point::new(12, -9), 8.),
            (Point::new(13, -9), 12.),
            (Point::new(14, -9), 19.),
            (Point::new(-15, -8), 7.),
            (Point::new(-14, -8), 7.),
            (Point::new(-13, -8), 7.),
            (Point::new(-12, -8), 6.),
            (Point::new(-11, -8), 3.),
            (Point::new(-10, -8), 3.),
            (Point::new(-9, -8), 4.),
            (Point::new(-8, -8), 4.),
            (Point::new(-7, -8), 6.),
            (Point::new(-6, -8), 9.),
            (Point::new(-5, -8), 9.),
            (Point::new(-4, -8), 8.),
            (Point::new(-3, -8), 6.),
            (Point::new(-2, -8), 3.),
            (Point::new(-1, -8), 6.),
            (Point::new(0, -8), 9.),
            (Point::new(1, -8), 12.),
            (Point::new(2, -8), 15.),
            (Point::new(3, -8), 17.),
            (Point::new(4, -8), 15.),
            (Point::new(5, -8), 9.),
            (Point::new(6, -8), 5.),
            (Point::new(7, -8), 3.),
            (Point::new(8, -8), 5.),
            (Point::new(9, -8), 5.),
            (Point::new(10, -8), 4.),
            (Point::new(11, -8), 5.),
            (Point::new(12, -8), 8.),
            (Point::new(13, -8), 12.),
            (Point::new(14, -8), 19.),
            (Point::new(-15, -7), 5.),
            (Point::new(-14, -7), 6.),
            (Point::new(-13, -7), 6.),
            (Point::new(-12, -7), 6.),
            (Point::new(-11, -7), 5.),
            (Point::new(-10, -7), 5.),
            (Point::new(-9, -7), 4.),
            (Point::new(-8, -7), 3.),
            (Point::new(-7, -7), 5.),
            (Point::new(-6, -7), 9.),
            (Point::new(-5, -7), 9.),
            (Point::new(-4, -7), 9.),
            (Point::new(-3, -7), 5.),
            (Point::new(-2, -7), 4.),
            (Point::new(-1, -7), 9.),
            (Point::new(0, -7), 14.),
            (Point::new(1, -7), 17.),
            (Point::new(2, -7), 22.),
            (Point::new(3, -7), 22.),
            (Point::new(4, -7), 12.),
            (Point::new(5, -7), 6.),
            (Point::new(6, -7), 4.),
            (Point::new(7, -7), 4.),
            (Point::new(8, -7), 7.),
            (Point::new(9, -7), 7.),
            (Point::new(10, -7), 6.),
            (Point::new(11, -7), 6.),
            (Point::new(12, -7), 8.),
            (Point::new(13, -7), 12.),
            (Point::new(14, -7), 17.),
            (Point::new(-15, -6), 5.),
            (Point::new(-14, -6), 5.),
            (Point::new(-13, -6), 6.),
            (Point::new(-12, -6), 6.),
            (Point::new(-11, -6), 6.),
            (Point::new(-10, -6), 6.),
            (Point::new(-9, -6), 6.),
            (Point::new(-8, -6), 4.),
            (Point::new(-7, -6), 3.),
            (Point::new(-6, -6), 5.),
            (Point::new(-5, -6), 8.),
            (Point::new(-4, -6), 7.),
            (Point::new(-3, -6), 4.),
            (Point::new(-2, -6), 5.),
            (Point::new(-1, -6), 11.),
            (Point::new(0, -6), 15.),
            (Point::new(1, -6), 17.),
            (Point::new(2, -6), 19.),
            (Point::new(3, -6), 16.),
            (Point::new(4, -6), 8.),
            (Point::new(5, -6), 5.),
            (Point::new(6, -6), 3.),
            (Point::new(7, -6), 5.),
            (Point::new(8, -6), 9.),
            (Point::new(9, -6), 11.),
            (Point::new(10, -6), 9.),
            (Point::new(11, -6), 9.),
            (Point::new(12, -6), 11.),
            (Point::new(13, -6), 13.),
            (Point::new(14, -6), 16.),
            (Point::new(-15, -5), 4.),
            (Point::new(-14, -5), 6.),
            (Point::new(-13, -5), 7.),
            (Point::new(-12, -5), 6.),
            (Point::new(-11, -5), 6.),
            (Point::new(-10, -5), 6.),
            (Point::new(-9, -5), 6.),
            (Point::new(-8, -5), 5.),
            (Point::new(-7, -5), 4.),
            (Point::new(-6, -5), 3.),
            (Point::new(-5, -5), 4.),
            (Point::new(-4, -5), 4.),
            (Point::new(-3, -5), 3.),
            (Point::new(-2, -5), 5.),
            (Point::new(-1, -5), 10.),
            (Point::new(0, -5), 14.),
            (Point::new(1, -5), 17.),
            (Point::new(2, -5), 16.),
            (Point::new(3, -5), 12.),
            (Point::new(4, -5), 7.),
            (Point::new(5, -5), 4.),
            (Point::new(6, -5), 3.),
            (Point::new(7, -5), 6.),
            (Point::new(8, -5), 12.),
            (Point::new(9, -5), 14.),
            (Point::new(10, -5), 12.),
            (Point::new(11, -5), 11.),
            (Point::new(12, -5), 12.),
            (Point::new(13, -5), 16.),
            (Point::new(14, -5), 16.),
            (Point::new(-15, -4), 4.),
            (Point::new(-14, -4), 7.),
            (Point::new(-13, -4), 8.),
            (Point::new(-12, -4), 6.),
            (Point::new(-11, -4), 5.),
            (Point::new(-10, -4), 5.),
            (Point::new(-9, -4), 6.),
            (Point::new(-8, -4), 6.),
            (Point::new(-7, -4), 5.),
            (Point::new(-6, -4), 4.),
            (Point::new(-5, -4), 3.),
            (Point::new(-4, -4), 3.),
            (Point::new(-3, -4), 3.),
            (Point::new(-2, -4), 4.),
            (Point::new(-1, -4), 8.),
            (Point::new(0, -4), 16.),
            (Point::new(1, -4), 20.),
            (Point::new(2, -4), 16.),
            (Point::new(3, -4), 10.),
            (Point::new(4, -4), 5.),
            (Point::new(5, -4), 3.),
            (Point::new(6, -4), 4.),
            (Point::new(7, -4), 8.),
            (Point::new(8, -4), 13.),
            (Point::new(9, -4), 13.),
            (Point::new(10, -4), 9.),
            (Point::new(11, -4), 8.),
            (Point::new(12, -4), 9.),
            (Point::new(13, -4), 14.),
            (Point::new(14, -4), 14.),
            (Point::new(-15, -3), 4.),
            (Point::new(-14, -3), 7.),
            (Point::new(-13, -3), 8.),
            (Point::new(-12, -3), 6.),
            (Point::new(-11, -3), 4.),
            (Point::new(-10, -3), 5.),
            (Point::new(-9, -3), 5.),
            (Point::new(-8, -3), 6.),
            (Point::new(-7, -3), 6.),
            (Point::new(-6, -3), 5.),
            (Point::new(-5, -3), 5.),
            (Point::new(-4, -3), 4.),
            (Point::new(-3, -3), 4.),
            (Point::new(-2, -3), 4.),
            (Point::new(-1, -3), 8.),
            (Point::new(0, -3), 14.),
            (Point::new(1, -3), 18.),
            (Point::new(2, -3), 12.),
            (Point::new(3, -3), 6.),
            (Point::new(4, -3), 3.),
            (Point::new(5, -3), 4.),
            (Point::new(6, -3), 6.),
            (Point::new(7, -3), 9.),
            (Point::new(8, -3), 11.),
            (Point::new(9, -3), 9.),
            (Point::new(10, -3), 6.),
            (Point::new(11, -3), 5.),
            (Point::new(12, -3), 7.),
            (Point::new(13, -3), 9.),
            (Point::new(14, -3), 11.),
            (Point::new(-15, -2), 3.),
            (Point::new(-14, -2), 6.),
            (Point::new(-13, -2), 7.),
            (Point::new(-12, -2), 6.),
            (Point::new(-11, -2), 4.),
            (Point::new(-10, -2), 4.),
            (Point::new(-9, -2), 4.),
            (Point::new(-8, -2), 5.),
            (Point::new(-7, -2), 6.),
            (Point::new(-6, -2), 6.),
            (Point::new(-5, -2), 5.),
            (Point::new(-4, -2), 4.),
            (Point::new(-3, -2), 4.),
            (Point::new(-2, -2), 4.),
            (Point::new(-1, -2), 6.),
            (Point::new(0, -2), 10.),
            (Point::new(1, -2), 10.),
            (Point::new(2, -2), 6.),
            (Point::new(3, -2), 3.),
            (Point::new(4, -2), 4.),
            (Point::new(5, -2), 5.),
            (Point::new(6, -2), 7.),
            (Point::new(7, -2), 9.),
            (Point::new(8, -2), 7.),
            (Point::new(9, -2), 5.),
            (Point::new(10, -2), 3.),
            (Point::new(11, -2), 3.),
            (Point::new(12, -2), 4.),
            (Point::new(13, -2), 6.),
            (Point::new(14, -2), 6.),
            (Point::new(-15, -1), 3.),
            (Point::new(-14, -1), 4.),
            (Point::new(-13, -1), 6.),
            (Point::new(-12, -1), 6.),
            (Point::new(-11, -1), 5.),
            (Point::new(-10, -1), 4.),
            (Point::new(-9, -1), 4.),
            (Point::new(-8, -1), 4.),
            (Point::new(-7, -1), 5.),
            (Point::new(-6, -1), 5.),
            (Point::new(-5, -1), 5.),
            (Point::new(-4, -1), 4.),
            (Point::new(-3, -1), 3.),
            (Point::new(-2, -1), 3.),
            (Point::new(-1, -1), 4.),
            (Point::new(0, -1), 5.),
            (Point::new(1, -1), 4.),
            (Point::new(2, -1), 3.),
            (Point::new(3, -1), 4.),
            (Point::new(4, -1), 5.),
            (Point::new(5, -1), 5.),
            (Point::new(6, -1), 5.),
            (Point::new(7, -1), 6.),
            (Point::new(8, -1), 4.),
            (Point::new(9, -1), 3.),
            (Point::new(10, -1), 3.),
            (Point::new(11, -1), 3.),
            (Point::new(12, -1), 3.),
            (Point::new(13, -1), 3.),
            (Point::new(14, -1), 3.),
            (Point::new(-15, 0), 3.),
            (Point::new(-14, 0), 4.),
            (Point::new(-13, 0), 6.),
            (Point::new(-12, 0), 6.),
            (Point::new(-11, 0), 6.),
            (Point::new(-10, 0), 5.),
            (Point::new(-9, 0), 5.),
            (Point::new(-8, 0), 4.),
            (Point::new(-7, 0), 4.),
            (Point::new(-6, 0), 4.),
            (Point::new(-5, 0), 3.),
            (Point::new(-4, 0), 3.),
            (Point::new(-3, 0), 3.),
            (Point::new(-2, 0), 4.),
            (Point::new(-1, 0), 5.),
            (Point::new(0, 0), 5.),
            (Point::new(1, 0), 5.),
            (Point::new(2, 0), 6.),
            (Point::new(3, 0), 5.),
            (Point::new(4, 0), 5.),
            (Point::new(5, 0), 4.),
            (Point::new(6, 0), 5.),
            (Point::new(7, 0), 5.),
            (Point::new(8, 0), 4.),
            (Point::new(9, 0), 3.),
            (Point::new(10, 0), 3.),
            (Point::new(11, 0), 3.),
            (Point::new(12, 0), 3.),
            (Point::new(13, 0), 3.),
            (Point::new(14, 0), 3.),
            (Point::new(-15, 1), 3.),
            (Point::new(-14, 1), 3.),
            (Point::new(-13, 1), 3.),
            (Point::new(-12, 1), 4.),
            (Point::new(-11, 1), 5.),
            (Point::new(-10, 1), 6.),
            (Point::new(-9, 1), 5.),
            (Point::new(-8, 1), 4.),
            (Point::new(-7, 1), 3.),
            (Point::new(-6, 1), 3.),
            (Point::new(-5, 1), 4.),
            (Point::new(-4, 1), 5.),
            (Point::new(-3, 1), 6.),
            (Point::new(-2, 1), 9.),
            (Point::new(-1, 1), 13.),
            (Point::new(0, 1), 14.),
            (Point::new(1, 1), 13.),
            (Point::new(2, 1), 9.),
            (Point::new(3, 1), 6.),
            (Point::new(4, 1), 5.),
            (Point::new(5, 1), 4.),
            (Point::new(6, 1), 4.),
            (Point::new(7, 1), 4.),
            (Point::new(8, 1), 3.),
            (Point::new(9, 1), 3.),
            (Point::new(10, 1), 3.),
            (Point::new(11, 1), 3.),
            (Point::new(12, 1), 3.),
            (Point::new(13, 1), 3.),
            (Point::new(14, 1), 3.),
            (Point::new(-15, 2), 3.),
            (Point::new(-14, 2), 4.),
            (Point::new(-13, 2), 4.),
            (Point::new(-12, 2), 3.),
            (Point::new(-11, 2), 4.),
            (Point::new(-10, 2), 5.),
            (Point::new(-9, 2), 4.),
            (Point::new(-8, 2), 3.),
            (Point::new(-7, 2), 4.),
            (Point::new(-6, 2), 6.),
            (Point::new(-5, 2), 8.),
            (Point::new(-4, 2), 9.),
            (Point::new(-3, 2), 13.),
            (Point::new(-2, 2), 19.),
            (Point::new(-1, 2), 23.),
            (Point::new(0, 2), 23.),
            (Point::new(1, 2), 19.),
            (Point::new(2, 2), 12.),
            (Point::new(3, 2), 8.),
            (Point::new(4, 2), 5.),
            (Point::new(5, 2), 4.),
            (Point::new(6, 2), 3.),
            (Point::new(7, 2), 3.),
            (Point::new(8, 2), 4.),
            (Point::new(9, 2), 4.),
            (Point::new(10, 2), 3.),
            (Point::new(11, 2), 3.),
            (Point::new(12, 2), 3.),
            (Point::new(13, 2), 3.),
            (Point::new(14, 2), 3.),
            (Point::new(-15, 3), 6.),
            (Point::new(-14, 3), 6.),
            (Point::new(-13, 3), 5.),
            (Point::new(-12, 3), 4.),
            (Point::new(-11, 3), 3.),
            (Point::new(-10, 3), 3.),
            (Point::new(-9, 3), 3.),
            (Point::new(-8, 3), 5.),
            (Point::new(-7, 3), 7.),
            (Point::new(-6, 3), 9.),
            (Point::new(-5, 3), 13.),
            (Point::new(-4, 3), 17.),
            (Point::new(-3, 3), 23.),
            (Point::new(-2, 3), 28.),
            (Point::new(-1, 3), 31.),
            (Point::new(0, 3), 25.),
            (Point::new(1, 3), 21.),
            (Point::new(2, 3), 17.),
            (Point::new(3, 3), 12.),
            (Point::new(4, 3), 8.),
            (Point::new(5, 3), 4.),
            (Point::new(6, 3), 3.),
            (Point::new(7, 3), 5.),
            (Point::new(8, 3), 6.),
            (Point::new(9, 3), 7.),
            (Point::new(10, 3), 5.),
            (Point::new(11, 3), 4.),
            (Point::new(12, 3), 4.),
            (Point::new(13, 3), 3.),
            (Point::new(14, 3), 3.),
            (Point::new(-15, 4), 9.),
            (Point::new(-14, 4), 8.),
            (Point::new(-13, 4), 6.),
            (Point::new(-12, 4), 5.),
            (Point::new(-11, 4), 5.),
            (Point::new(-10, 4), 5.),
            (Point::new(-9, 4), 6.),
            (Point::new(-8, 4), 7.),
            (Point::new(-7, 4), 9.),
            (Point::new(-6, 4), 12.),
            (Point::new(-5, 4), 17.),
            (Point::new(-4, 4), 25.),
            (Point::new(-3, 4), 33.),
            (Point::new(-2, 4), 33.),
            (Point::new(-1, 4), 31.),
            (Point::new(0, 4), 25.),
            (Point::new(1, 4), 21.),
            (Point::new(2, 4), 21.),
            (Point::new(3, 4), 16.),
            (Point::new(4, 4), 11.),
            (Point::new(5, 4), 6.),
            (Point::new(6, 4), 3.),
            (Point::new(7, 4), 4.),
            (Point::new(8, 4), 6.),
            (Point::new(9, 4), 8.),
            (Point::new(10, 4), 8.),
            (Point::new(11, 4), 6.),
            (Point::new(12, 4), 6.),
            (Point::new(13, 4), 7.),
            (Point::new(14, 4), 8.),
            (Point::new(-15, 5), 9.),
            (Point::new(-14, 5), 8.),
            (Point::new(-13, 5), 7.),
            (Point::new(-12, 5), 7.),
            (Point::new(-11, 5), 9.),
            (Point::new(-10, 5), 11.),
            (Point::new(-9, 5), 11.),
            (Point::new(-8, 5), 9.),
            (Point::new(-7, 5), 11.),
            (Point::new(-6, 5), 14.),
            (Point::new(-5, 5), 21.),
            (Point::new(-4, 5), 31.),
            (Point::new(-3, 5), 37.),
            (Point::new(-2, 5), 33.),
            (Point::new(-1, 5), 25.),
            (Point::new(0, 5), 21.),
            (Point::new(1, 5), 19.),
            (Point::new(2, 5), 19.),
            (Point::new(3, 5), 17.),
            (Point::new(4, 5), 11.),
            (Point::new(5, 5), 7.),
            (Point::new(6, 5), 5.),
            (Point::new(7, 5), 4.),
            (Point::new(8, 5), 4.),
            (Point::new(9, 5), 6.),
            (Point::new(10, 5), 7.),
            (Point::new(11, 5), 6.),
            (Point::new(12, 5), 8.),
            (Point::new(13, 5), 12.),
            (Point::new(14, 5), 17.),
            (Point::new(-15, 6), 8.),
            (Point::new(-14, 6), 9.),
            (Point::new(-13, 6), 9.),
            (Point::new(-12, 6), 11.),
            (Point::new(-11, 6), 13.),
            (Point::new(-10, 6), 13.),
            (Point::new(-9, 6), 13.),
            (Point::new(-8, 6), 12.),
            (Point::new(-7, 6), 13.),
            (Point::new(-6, 6), 17.),
            (Point::new(-5, 6), 23.),
            (Point::new(-4, 6), 31.),
            (Point::new(-3, 6), 31.),
            (Point::new(-2, 6), 25.),
            (Point::new(-1, 6), 21.),
            (Point::new(0, 6), 16.),
            (Point::new(1, 6), 16.),
            (Point::new(2, 6), 17.),
            (Point::new(3, 6), 16.),
            (Point::new(4, 6), 11.),
            (Point::new(5, 6), 7.),
            (Point::new(6, 6), 5.),
            (Point::new(7, 6), 5.),
            (Point::new(8, 6), 3.),
            (Point::new(9, 6), 4.),
            (Point::new(10, 6), 5.),
            (Point::new(11, 6), 6.),
            (Point::new(12, 6), 9.),
            (Point::new(13, 6), 14.),
            (Point::new(14, 6), 17.),
            (Point::new(-15, 7), 7.),
            (Point::new(-14, 7), 9.),
            (Point::new(-13, 7), 12.),
            (Point::new(-12, 7), 14.),
            (Point::new(-11, 7), 14.),
            (Point::new(-10, 7), 14.),
            (Point::new(-9, 7), 14.),
            (Point::new(-8, 7), 16.),
            (Point::new(-7, 7), 21.),
            (Point::new(-6, 7), 25.),
            (Point::new(-5, 7), 28.),
            (Point::new(-4, 7), 25.),
            (Point::new(-3, 7), 25.),
            (Point::new(-2, 7), 23.),
            (Point::new(-1, 7), 19.),
            (Point::new(0, 7), 14.),
            (Point::new(1, 7), 13.),
            (Point::new(2, 7), 14.),
            (Point::new(3, 7), 16.),
            (Point::new(4, 7), 12.),
            (Point::new(5, 7), 8.),
            (Point::new(6, 7), 6.),
            (Point::new(7, 7), 5.),
            (Point::new(8, 7), 4.),
            (Point::new(9, 7), 4.),
            (Point::new(10, 7), 5.),
            (Point::new(11, 7), 7.),
            (Point::new(12, 7), 10.),
            (Point::new(13, 7), 12.),
            (Point::new(14, 7), 11.),
            (Point::new(-15, 8), 8.),
            (Point::new(-14, 8), 9.),
            (Point::new(-13, 8), 12.),
            (Point::new(-12, 8), 14.),
            (Point::new(-11, 8), 16.),
            (Point::new(-10, 8), 16.),
            (Point::new(-9, 8), 17.),
            (Point::new(-8, 8), 23.),
            (Point::new(-7, 8), 31.),
            (Point::new(-6, 8), 33.),
            (Point::new(-5, 8), 31.),
            (Point::new(-4, 8), 25.),
            (Point::new(-3, 8), 21.),
            (Point::new(-2, 8), 21.),
            (Point::new(-1, 8), 19.),
            (Point::new(0, 8), 13.),
            (Point::new(1, 8), 11.),
            (Point::new(2, 8), 13.),
            (Point::new(3, 8), 16.),
            (Point::new(4, 8), 13.),
            (Point::new(5, 8), 9.),
            (Point::new(6, 8), 6.),
            (Point::new(7, 8), 6.),
            (Point::new(8, 8), 5.),
            (Point::new(9, 8), 3.),
            (Point::new(10, 8), 5.),
            (Point::new(11, 8), 8.),
            (Point::new(12, 8), 9.),
            (Point::new(13, 8), 8.),
            (Point::new(14, 8), 6.),
            (Point::new(-15, 9), 9.),
            (Point::new(-14, 9), 9.),
            (Point::new(-13, 9), 11.),
            (Point::new(-12, 9), 13.),
            (Point::new(-11, 9), 16.),
            (Point::new(-10, 9), 21.),
            (Point::new(-9, 9), 23.),
            (Point::new(-8, 9), 28.),
            (Point::new(-7, 9), 33.),
            (Point::new(-6, 9), 33.),
            (Point::new(-5, 9), 28.),
            (Point::new(-4, 9), 23.),
            (Point::new(-3, 9), 19.),
            (Point::new(-2, 9), 19.),
            (Point::new(-1, 9), 17.),
            (Point::new(0, 9), 12.),
            (Point::new(1, 9), 9.),
            (Point::new(2, 9), 11.),
            (Point::new(3, 9), 13.),
            (Point::new(4, 9), 12.),
            (Point::new(5, 9), 9.),
            (Point::new(6, 9), 7.),
            (Point::new(7, 9), 7.),
            (Point::new(8, 9), 6.),
            (Point::new(9, 9), 4.),
            (Point::new(10, 9), 4.),
            (Point::new(11, 9), 6.),
            (Point::new(12, 9), 6.),
            (Point::new(13, 9), 4.),
            (Point::new(14, 9), 3.),
            (Point::new(-15, 10), 9.),
            (Point::new(-14, 10), 8.),
            (Point::new(-13, 10), 8.),
            (Point::new(-12, 10), 9.),
            (Point::new(-11, 10), 14.),
            (Point::new(-10, 10), 21.),
            (Point::new(-9, 10), 28.),
            (Point::new(-8, 10), 28.),
            (Point::new(-7, 10), 25.),
            (Point::new(-6, 10), 23.),
            (Point::new(-5, 10), 21.),
            (Point::new(-4, 10), 17.),
            (Point::new(-3, 10), 14.),
            (Point::new(-2, 10), 13.),
            (Point::new(-1, 10), 12.),
            (Point::new(0, 10), 9.),
            (Point::new(1, 10), 8.),
            (Point::new(2, 10), 9.),
            (Point::new(3, 10), 11.),
            (Point::new(4, 10), 12.),
            (Point::new(5, 10), 9.),
            (Point::new(6, 10), 8.),
            (Point::new(7, 10), 8.),
            (Point::new(8, 10), 8.),
            (Point::new(9, 10), 5.),
            (Point::new(10, 10), 3.),
            (Point::new(11, 10), 4.),
            (Point::new(12, 10), 4.),
            (Point::new(13, 10), 3.),
            (Point::new(14, 10), 3.),
            (Point::new(-15, 11), 11.),
            (Point::new(-14, 11), 8.),
            (Point::new(-13, 11), 6.),
            (Point::new(-12, 11), 8.),
            (Point::new(-11, 11), 12.),
            (Point::new(-10, 11), 17.),
            (Point::new(-9, 11), 23.),
            (Point::new(-8, 11), 23.),
            (Point::new(-7, 11), 21.),
            (Point::new(-6, 11), 17.),
            (Point::new(-5, 11), 16.),
            (Point::new(-4, 11), 12.),
            (Point::new(-3, 11), 9.),
            (Point::new(-2, 11), 7.),
            (Point::new(-1, 11), 6.),
            (Point::new(0, 11), 5.),
            (Point::new(1, 11), 5.),
            (Point::new(2, 11), 5.),
            (Point::new(3, 11), 8.),
            (Point::new(4, 11), 11.),
            (Point::new(5, 11), 11.),
            (Point::new(6, 11), 9.),
            (Point::new(7, 11), 9.),
            (Point::new(8, 11), 8.),
            (Point::new(9, 11), 5.),
            (Point::new(10, 11), 3.),
            (Point::new(11, 11), 3.),
            (Point::new(12, 11), 3.),
            (Point::new(13, 11), 4.),
            (Point::new(14, 11), 5.),
            (Point::new(-15, 12), 13.),
            (Point::new(-14, 12), 9.),
            (Point::new(-13, 12), 8.),
            (Point::new(-12, 12), 8.),
            (Point::new(-11, 12), 9.),
            (Point::new(-10, 12), 14.),
            (Point::new(-9, 12), 21.),
            (Point::new(-8, 12), 23.),
            (Point::new(-7, 12), 17.),
            (Point::new(-6, 12), 14.),
            (Point::new(-5, 12), 13.),
            (Point::new(-4, 12), 11.),
            (Point::new(-3, 12), 6.),
            (Point::new(-2, 12), 4.),
            (Point::new(-1, 12), 3.),
            (Point::new(0, 12), 3.),
            (Point::new(1, 12), 4.),
            (Point::new(2, 12), 3.),
            (Point::new(3, 12), 5.),
            (Point::new(4, 12), 9.),
            (Point::new(5, 12), 11.),
            (Point::new(6, 12), 11.),
            (Point::new(7, 12), 11.),
            (Point::new(8, 12), 8.),
            (Point::new(9, 12), 5.),
            (Point::new(10, 12), 3.),
            (Point::new(11, 12), 3.),
            (Point::new(12, 12), 4.),
            (Point::new(13, 12), 6.),
            (Point::new(14, 12), 7.),
            (Point::new(-15, 13), 19.),
            (Point::new(-14, 13), 14.),
            (Point::new(-13, 13), 13.),
            (Point::new(-12, 13), 11.),
            (Point::new(-11, 13), 11.),
            (Point::new(-10, 13), 14.),
            (Point::new(-9, 13), 19.),
            (Point::new(-8, 13), 17.),
            (Point::new(-7, 13), 13.),
            (Point::new(-6, 13), 11.),
            (Point::new(-5, 13), 11.),
            (Point::new(-4, 13), 9.),
            (Point::new(-3, 13), 6.),
            (Point::new(-2, 13), 4.),
            (Point::new(-1, 13), 3.),
            (Point::new(0, 13), 4.),
            (Point::new(1, 13), 5.),
            (Point::new(2, 13), 3.),
            (Point::new(3, 13), 4.),
            (Point::new(4, 13), 7.),
            (Point::new(5, 13), 9.),
            (Point::new(6, 13), 12.),
            (Point::new(7, 13), 12.),
            (Point::new(8, 13), 8.),
            (Point::new(9, 13), 5.),
            (Point::new(10, 13), 4.),
            (Point::new(11, 13), 5.),
            (Point::new(12, 13), 7.),
            (Point::new(13, 13), 9.),
            (Point::new(14, 13), 9.),
            (Point::new(-15, 14), 25.),
            (Point::new(-14, 14), 23.),
            (Point::new(-13, 14), 21.),
            (Point::new(-12, 14), 17.),
            (Point::new(-11, 14), 14.),
            (Point::new(-10, 14), 14.),
            (Point::new(-9, 14), 14.),
            (Point::new(-8, 14), 12.),
            (Point::new(-7, 14), 8.),
            (Point::new(-6, 14), 7.),
            (Point::new(-5, 14), 9.),
            (Point::new(-4, 14), 9.),
            (Point::new(-3, 14), 8.),
            (Point::new(-2, 14), 5.),
            (Point::new(-1, 14), 4.),
            (Point::new(0, 14), 4.),
            (Point::new(1, 14), 5.),
            (Point::new(2, 14), 3.),
            (Point::new(3, 14), 4.),
            (Point::new(4, 14), 5.),
            (Point::new(5, 14), 7.),
            (Point::new(6, 14), 11.),
            (Point::new(7, 14), 11.),
            (Point::new(8, 14), 8.),
            (Point::new(9, 14), 5.),
            (Point::new(10, 14), 4.),
            (Point::new(11, 14), 5.),
            (Point::new(12, 14), 8.),
            (Point::new(13, 14), 11.),
            (Point::new(14, 14), 12.)
        ];

        let finder = Pathfinder::new(tilemap, (0, 0).into(), (2, 2).into());
        let path = finder.find();

        assert_eq!(path.len(), 4);
    }
}
