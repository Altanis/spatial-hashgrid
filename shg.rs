use std::collections::{HashMap, HashSet};

const CELL_SIZE: i32 = 256;
const CELL_SIZE_LOG2: i32 = CELL_SIZE.trailing_zeros() as i32;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pair(i32, i32);

impl Pair {
    fn from_coords(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    fn from_index(index: usize, grid_width: usize) -> Self {
        let x = (index % grid_width) as i32;
        let y = (index / grid_width) as i32;
        Self(x, y)
    }

    fn to_index(&self, grid_width: usize) -> usize {
        (self.1 as usize * grid_width) + self.0 as usize
    }
}

struct SpatialHashGrid {
    grid: HashMap<Pair, Vec<usize>>,
    dimensions: HashMap<usize, (i32, i32)>,
    ids: HashMap<usize, HashSet<usize>>,
}

impl SpatialHashGrid {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            dimensions: HashMap::new(),
            ids: HashMap::new(),
        }
    }

    fn insert(&mut self, x: i32, y: i32, width: i32, height: i32, id: usize) {
        let x1 = x >> CELL_SIZE_LOG2;
        let y1 = y >> CELL_SIZE_LOG2;
        let x2 = (x + width) >> CELL_SIZE_LOG2;
        let y2 = (y + height) >> CELL_SIZE_LOG2;

        for i in x1..=x2 {
            for j in y1..=y2 {
                let pair = Pair::from_coords(i, j);
                let index = pair.to_index(1 << CELL_SIZE_LOG2);
                self.grid.entry(pair).or_default().push(id);
                self.dimensions.insert(id, (width, height));
                self.ids.entry(index).or_default().insert(id);
            }
        }
    }

    fn query(&self, x: i32, y: i32, width: i32, height: i32) -> Vec<usize> {
        let mut result = HashSet::new();

        let x1 = x >> CELL_SIZE_LOG2;
        let y1 = y >> CELL_SIZE_LOG2;
        let x2 = (x + width) >> CELL_SIZE_LOG2;
        let y2 = (y + height) >> CELL_SIZE_LOG2;

        for i in x1..=x2 {
            for j in y1..=y2 {
                let pair = Pair::from_coords(i, j);
                if let Some(ids) = self.grid.get(&pair) {
                    for &id in ids {
                        let (entity_width, entity_height) = self.dimensions.get(&id).unwrap();
                        let entity_x = (i << CELL_SIZE_LOG2) as i32;
                        let entity_y = (j << CELL_SIZE_LOG2) as i32;
                        if self.collides_with(entity_x, entity_y, *entity_width, *entity_height, x, y, width, height) {
                            let index = Pair::from_coords(entity_x >> CELL_SIZE_LOG2, entity_y >> CELL_SIZE_LOG2).to_index(1 << CELL_SIZE_LOG2);
                            result.extend(self.ids.get(&index).unwrap());
                        }
                    }
                }
            }
        }

        result.into_iter().collect()
    }

    fn collides_with(&self, x1: i32, y1: i32, w1: i32, h1: i32, x2: i32, y2: i32, w2: i32, h2: i32) -> bool {
        x1 < x2 + w2 &&
        x1 + w1 > x2 &&
        y1 < y2 + h2 &&
        y1 + h1 > y2
    }
}

fn main() {
	/* Instantiate grid. */
    let mut grid = SpatialHashGrid::new();

	/* Insert entities. */
    grid.insert(0, 0, 10, 10, 0);
    grid.insert(9, 9, 10, 10, 1);
    grid.insert(1, 1, 100, 100, 1);

	/* Query entities. */
    let mut result = grid.query(0, 0, 10, 10);

    println!("{:?}", result);
}