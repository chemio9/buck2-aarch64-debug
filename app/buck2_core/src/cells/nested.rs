/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use allocative::Allocative;

use crate::cells::cell_path::CellPathRef;
use crate::cells::cell_root_path::CellRootPath;
use crate::cells::name::CellName;
use crate::cells::paths::CellRelativePath;
use crate::cells::paths::CellRelativePathBuf;
use crate::cells::unchecked_cell_rel_path::UncheckedCellRelativePath;

/// Paths to cells which reside inside current cell.
///
/// Target labels cannot cross cell boundaries. This utility helps to identify such targets.
#[derive(Eq, PartialEq, Debug, Allocative, Clone)]
pub struct NestedCells {
    paths: Vec<(CellRelativePathBuf, CellName)>,
}

impl NestedCells {
    fn from_cell_paths_relative_to_this_cell<'a>(
        paths: impl IntoIterator<Item = (&'a CellRelativePath, CellName)>,
    ) -> NestedCells {
        let mut paths: Vec<_> = paths.into_iter().filter(|(p, _)| !p.is_empty()).collect();

        let mut final_paths: Vec<(CellRelativePathBuf, CellName)> = Vec::new();

        paths.sort();

        // Remove redundant paths, e.g. if `paths` contains `a` and `a/b`, then `a/b` is redundant.
        for (path, cell_name) in paths {
            if let Some((last_path, _)) = final_paths.last() {
                if path.starts_with(last_path) {
                    continue;
                }
            }

            final_paths.push((path.to_owned(), cell_name));
        }

        NestedCells { paths: final_paths }
    }

    pub fn from_cell_roots(
        all_cells: &[(CellName, &CellRootPath)],
        this_cell: &CellRootPath,
    ) -> NestedCells {
        Self::from_cell_paths_relative_to_this_cell(all_cells.iter().filter_map(
            |(cell_name, cell_root_path)| {
                let Some(path_relative_to_this_cell) = cell_root_path.strip_prefix_opt(this_cell)
                else {
                    return None;
                };

                Some((
                    CellRelativePath::new(path_relative_to_this_cell),
                    *cell_name,
                ))
            },
        ))
    }

    pub fn matches<'a, 'b>(
        &'a self,
        path: &'b UncheckedCellRelativePath,
    ) -> Option<(
        &'a CellRelativePath,
        CellName,
        &'b UncheckedCellRelativePath,
    )> {
        for (cell_path, cell_name) in &self.paths {
            if let Some(rem) = path.remove_prefix(cell_path) {
                return Some((cell_path, *cell_name, rem));
            }
        }
        None
    }

    pub(crate) fn matches_checked<'a, 'b>(
        &'a self,
        path: &'b CellRelativePath,
    ) -> Option<(&'a CellRelativePath, CellPathRef<'b>)> {
        self.matches(UncheckedCellRelativePath::new(path))
            .map(|(cell_path, cell_name, rem)| {
                (
                    cell_path,
                    CellPathRef::new(cell_name, CellRelativePath::unchecked_new(rem.as_str())),
                )
            })
    }

    pub(crate) fn check_empty(&self) -> Option<CellName> {
        self.paths.first().map(|(_, cell_name)| *cell_name)
    }
}

#[cfg(test)]
mod tests {

    use crate::cells::name::CellName;
    use crate::cells::nested::NestedCells;
    use crate::cells::paths::CellRelativePath;
    use crate::cells::paths::CellRelativePathBuf;

    #[test]
    fn test_nested_cells() {
        let nested_cells = NestedCells::from_cell_paths_relative_to_this_cell([
            (
                CellRelativePath::testing_new("cell_a/cell_b/cell_c"),
                CellName::testing_new("abc"),
            ),
            (
                CellRelativePath::testing_new("cell_a"),
                CellName::testing_new("a"),
            ),
            (
                CellRelativePath::testing_new("cell_x/cell_y"),
                CellName::testing_new("xy"),
            ),
            (
                CellRelativePath::testing_new("cell_a/cell_b"),
                CellName::testing_new("ab"),
            ),
        ]);

        assert_eq!(
            NestedCells {
                paths: vec![
                    (
                        CellRelativePathBuf::try_from("cell_a".to_owned()).unwrap(),
                        CellName::testing_new("a")
                    ),
                    (
                        CellRelativePathBuf::try_from("cell_x/cell_y".to_owned()).unwrap(),
                        CellName::testing_new("xy")
                    )
                ]
            },
            nested_cells,
        );
    }
}
