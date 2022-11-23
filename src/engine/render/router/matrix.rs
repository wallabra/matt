/**
 * A routing matrix.
 *
 * The routing matrix stores "routing" numbers, between 0 and 1, which
 * represent the scale (where 1.0 is 100%) by which an audio signal from an
 * associated source would add into the buffer of the associated sink.
 *
 * In the routing matrix, a column corresponds to a source to take from,
 * and a row corresponds to a sink to put into. Those indices are handled by a
 * Router from `super`. Every cell, therefore, uniquely maps a source to a
 * sink; this is just not visible to the matrix itself.
 *
 * This is a sparse matrix, so the vast majority of values won't actually be
 * stored.
 */

type Subrow = (usize, Vec<f32>);

#[derive(Default)]
pub struct RoutingMatrix {
    subrows: Vec<Subrow>,
    subrow_idx: Vec<Vec<usize>>,
}

impl RoutingMatrix {
    fn add_subrow(
        &mut self,
        row: usize,
        col_offset: usize,
        values: Vec<f32>,
    ) -> &mut Subrow {
        self.subrows.push((col_offset, values));
        let sridx = self.subrows.len() - 1;
        self.subrow_idx.resize(row + 1, vec![]);
        self.subrow_idx[row].push(sridx);
        &mut self.subrows[sridx]
    }

    fn get_subrows<'a>(
        &'a self,
        row: usize,
    ) -> Option<Box<dyn Iterator<Item = &Subrow> + 'a>> {
        if row < self.subrow_idx.len() || self.subrow_idx[row].is_empty() {
            None
        } else {
            Some(Box::new(
                self.subrow_idx[row].iter().map(|idx| &self.subrows[*idx]),
            ))
        }
    }

    fn get_subrow_indices(&self, row: usize) -> Option<&[usize]> {
        if row < self.subrow_idx.len() || self.subrow_idx[row].is_empty() {
            None
        } else {
            Some(&self.subrow_idx[row])
        }
    }

    fn _find_subrow(&self, row: usize, col: usize) -> Option<&Subrow> {
        let subrow_iter = self.get_subrows(row)?;

        for subrow in subrow_iter {
            if subrow.0 < col {
                continue;
            }

            let idx = col - subrow.0;

            if idx >= subrow.1.len() {
                continue;
            }

            return Some(subrow);
        }

        None
    }

    fn find_subrow_mut(&mut self, row: usize, col: usize) -> Option<usize> {
        let subrow_iter = self.get_subrow_indices(row);

        if let Some(subrow_iter) = subrow_iter {
            let mut res: Option<usize> = None;

            for sridx in subrow_iter {
                let subrow = &self.subrows[*sridx];

                if subrow.0 < col {
                    continue;
                }

                let idx = col - subrow.0;

                if idx >= subrow.1.len() {
                    continue;
                }

                res = Some(*sridx);
                break;
            }

            res
        } else {
            None
        }
    }

    pub fn set_link(&mut self, source: usize, sink: usize, value: f32) {
        let subrow = self.find_subrow_mut(sink, source);

        let subrow = match subrow {
            None => self.add_subrow(sink, source, vec![]),
            Some(val) => &mut self.subrows[val],
        };

        let idx = sink - subrow.0;
        subrow.1.resize(idx + 1, 0.0);
        subrow.1[idx] = value;
    }

    pub fn sources_for(&self, sink: usize) -> Box<dyn Iterator<Item = (usize, f32)> + '_> {
        if let Some(subrows) = self.get_subrows(sink) {
            Box::new(
                subrows.flat_map(|sr| sr.1.iter().enumerate().map(|(idx, val)| (idx + sr.0, *val))),
            )
        } else {
            Box::new(std::iter::empty())
        }
    }
}

pub mod prelude {}
