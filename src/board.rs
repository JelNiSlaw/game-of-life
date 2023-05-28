use std::cmp::Ordering;

use crate::quad_tree::QuadTree;

const DIRECTIONS: [(usize, usize); 8] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (1, 0),
    (1, 2),
    (2, 0),
    (2, 1),
    (2, 2),
];

#[derive(Clone)]
pub struct Board {
    time: u32,
    state: QuadTree<bool>,
}

impl Board {
    pub fn new(size: (usize, usize), time: u32) -> Self {
        Self {
            time,
            state: QuadTree::new(size, (0, 0), false),
        }
    }

    pub fn set_cell(&mut self, position: (usize, usize), value: bool) {
        self.state.insert(position, value);
    }

    pub fn next_state(&self, new_size: (usize, usize)) -> Self {
        let mut counts = QuadTree::new(new_size, (0, 0), 0);

        for (x, y) in self.cells() {
            for dir in DIRECTIONS {
                if dir.0 == 0 && x == 0
                    || dir.1 == 0 && y == 0
                    || dir.0 == 2 && x == self.size().0 - 1
                    || dir.1 == 2 && y == self.size().1 - 1
                {
                    continue;
                }

                let position = ((x + dir.0 - 1), (y + dir.1 - 1));

                if position.0 >= new_size.0 || position.1 >= new_size.1 {
                    continue;
                }

                match counts.get_mut(position) {
                    Some(count) => *count += 1,
                    None => counts.insert(position, 1),
                }
            }
        }

        let mut new_board = Board::new(new_size, self.time + 1);

        for ((x, y), count) in counts.all_nodes() {
            if count == 0 {
                continue;
            }

            let value = match count.cmp(&2) {
                Ordering::Less => false,
                Ordering::Equal => self.state.get((x, y)).unwrap_or(false),
                Ordering::Greater => count == 3,
            };

            new_board.set_cell((x, y), value);
        }

        new_board
    }

    pub fn cells(&self) -> impl Iterator<Item = (usize, usize)> {
        self.state
            .all_nodes()
            .into_iter()
            .filter(|(_, value)| *value)
            .map(|(position, _)| position)
    }

    pub fn time(&self) -> u32 {
        self.time
    }

    pub fn size(&self) -> (usize, usize) {
        self.state.size()
    }
}
