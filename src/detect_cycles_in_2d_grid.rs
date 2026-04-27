use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Pos(usize, usize);

pub struct Solution {}
impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let mut items_pos: HashMap<char, HashSet<Pos>> = HashMap::new();

        for (i_row, row) in grid.iter().enumerate() {
            for (i_cell, cell) in row.iter().enumerate() {
                items_pos
                    .entry(*cell)
                    .and_modify(|v| {
                        v.insert(Pos(i_row, i_cell));
                    })
                    .or_insert_with(|| {
                        let mut h = HashSet::new();
                        h.insert(Pos(i_row, i_cell));
                        h
                    });
            }
        }

        for (_, mut item_pos) in items_pos {
            if item_pos.len() < 4 {
                continue;
            }

            let pivot = *item_pos.iter().next().unwrap();
            let mut pivot = item_pos.take(&pivot).unwrap();

            let mut curr = pivot;
            let mut i = 1;

            loop {
                let mut neighbors = vec![Pos(curr.0, curr.1 + 1), Pos(curr.0 + 1, curr.1)];

                if curr.0 != 0 {
                    neighbors.push(Pos(curr.0 - 1, curr.1));
                }
                if curr.1 != 0 {
                    neighbors.push(Pos(curr.0, curr.1 - 1));
                }

                if i > 3 && neighbors.contains(&pivot) {
                    return true;
                }

                if let Some(pos) = neighbors.iter().find_map(|p| item_pos.get(p).copied()) {
                    curr = item_pos.take(&pos).unwrap();
                    i += 1;
                } else if item_pos.len() < 4 {
                    break;
                } else {
                    let t_pivot = *item_pos.iter().next().unwrap();
                    pivot = item_pos.take(&t_pivot).unwrap();
                    curr = pivot;
                    i = 1;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests() {
        /* assert!(Solution::contains_cycle(vec![
            vec!['h', 'a', 'a', 'c', 'x'],
            vec!['g', 'a', 'a', 'g', 'p'],
            vec!['p', 'h', 'b', 'x', 'x'],
            vec!['c', 'e', 'a', 'b', 'e'],
            vec!['f', 'e', 'f', 'b', 'f'],
        ]));

        assert!(Solution::contains_cycle(vec![
            vec!['c', 'c', 'c', 'a'],
            vec!['c', 'd', 'c', 'c'],
            vec!['c', 'c', 'e', 'c'],
            vec!['f', 'c', 'c', 'c'],
        ])); */

        assert!(Solution::contains_cycle(vec![
            vec!['c', 'a', 'd'],
            vec!['a', 'a', 'a'],
            vec!['a', 'a', 'd'],
            vec!['a', 'c', 'd'],
            vec!['a', 'b', 'c'],
        ]));
    }
/*
    #[test]
    fn tests_invalid() {
        assert!(!Solution::contains_cycle(vec![
            vec!['a', 'b', 'b'],
            vec!['b', 'z', 'b'],
            vec!['b', 'b', 'a'],
        ]));
    } */
}
