#![allow(dead_code, unused_variables)]

use nalgebra::{DMatrix, Dyn, MatrixView, OMatrix, SymmetricEigen, U1, OVector};
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

pub mod graph_type;

const EPSILON: f64 = 0.001;

#[derive(Debug, Clone)]
pub struct LapGraph<T: graph_type::GraphType> {
    // f64 is actually quite useful here, despite its scariness
    laplacian: OMatrix<f64, Dyn, Dyn>,
    vertex_count: usize,
    eigen_cache: Option<SymmetricEigen<f64, Dyn>>,
    eigenvalue_cache: Option<OVector<f64, Dyn>>,
    _p: PhantomData<T>,
}

impl<T: graph_type::GraphType> LapGraph<T> {
    pub fn empty(vertex_count: usize) -> Self {
        Self {
            laplacian: DMatrix::<f64>::zeros(vertex_count, vertex_count),
            vertex_count,
            eigen_cache: None,
            eigenvalue_cache: None,
            _p: PhantomData
        }
    }

    pub fn complete(vertex_count: usize) -> Self {
        let mut laplacian = DMatrix::<f64>::from_element(vertex_count, vertex_count, -1.0);

        laplacian.fill_diagonal((vertex_count as f64) - 1.0);

        Self {
            laplacian,
            vertex_count,
            eigen_cache: None,
            eigenvalue_cache: None,
            _p: PhantomData
        }
    }

    pub fn add_edge(&mut self, i: usize, j: usize) -> bool {
        debug_assert!(
            i != j,
            "LapGraph::<{}>::add_edge does not support self loops",
            self.vertex_count
        );

        debug_assert!(
            i < self.vertex_count && j < self.vertex_count,
            "LapGraph::<{}>::add_edge indices must lie in [0, {})",
            self.vertex_count,
            self.vertex_count
        );

        if T::MULTI_EDGES || self.laplacian[(i, j)] == 0.0 {
            self.laplacian[(i, j)] -= 1.0;
            self.laplacian[(j, i)] -= 1.0;
            self.laplacian[(i, i)] += 1.0;
            self.laplacian[(j, j)] += 1.0;

            self.eigen_cache = None;
            self.eigenvalue_cache = None;
            true
        }else {
            false
        }
    }

    pub fn remove_edge(&mut self, i: usize, j: usize) -> bool {
        debug_assert!(
            i != j,
            "LapGraph::<{}>::remove_edge does not support self loops",
            self.vertex_count
        );
        debug_assert!(
            i < self.vertex_count && j < self.vertex_count,
            "LapGraph::<{}>::remove_edge indices must lie in [0, {})",
            self.vertex_count,
            self.vertex_count
        );

        if self.laplacian[(i, j)] != 0.0 {
            self.laplacian[(i, j)] += 1.0;
            self.laplacian[(j, i)] += 1.0;
            self.laplacian[(i, i)] -= 1.0;
            self.laplacian[(j, j)] -= 1.0;

            self.eigen_cache = None;
            self.eigenvalue_cache = None;
            true
        }else {
            false
        }
    }

    pub fn eigenvalues(&mut self) -> MatrixView<'_, f64, Dyn, Dyn, U1, Dyn> {
        if self.eigenvalue_cache.is_none() {
            let _ = self
                .eigenvalue_cache
                .insert(self.laplacian.symmetric_eigenvalues());
        }
        self.eigenvalue_cache.as_ref().unwrap().as_view()
    }

    pub fn eigenvectors(&mut self) -> MatrixView<'_, f64, Dyn, Dyn, U1, Dyn> {
        if self.eigen_cache.is_none() {
            let _ = self
                .eigen_cache
                .insert(self.laplacian.clone_owned().symmetric_eigen());

            // Overwrite eigenvalue_cache?
        }
        self.eigen_cache.as_ref().unwrap().eigenvectors.as_view()
    }

    // Notably, returns false for empty graphs
    pub fn connected(&mut self) -> bool {
        self.count_components() == 1
    }

    // Tests if the edge ab is adjacent to the graph
    pub fn is_adjacent(&self, a: usize, b: usize) -> bool {
        self.laplacian[(a, a)] != 0.0 || self.laplacian[(b, b)] != 0.0
    }

    // Again, following the precedence that degree 0 vertices
    // do not exist
    pub fn count_components(&mut self) -> usize {
        let num_zeros = self
            .eigenvalues()
            .iter()
            .fold(0, |acc, &x| if x < EPSILON { acc + 1 } else { acc });

        let num_isolates =
            self.laplacian.diagonal().iter().fold(
                0,
                |acc, &x| {
                    if x == 0.0 {
                        acc + 1
                    } else {
                        acc
                    }
                },
            );

        num_zeros - num_isolates
    }

    pub fn fully_connected(&mut self) -> bool {
        let num_zeros = self
            .eigenvalues()
            .iter()
            .fold(0, |acc, &x| if x < EPSILON { acc + 1 } else { acc });

        num_zeros == 1
    }

    pub fn count_spanning_trees(&mut self) -> f64 {
        let (approx, non_zeros) =
            self.eigenvalues()
                .iter()
                .fold((1.0, 0), |(acc_prod, acc_nz), &x| {
                    if x > EPSILON {
                        (acc_prod * x, acc_nz + 1)
                    } else {
                        (acc_prod, acc_nz)
                    }
                });

        let expected_non_zeros =
            self.laplacian.diagonal().iter().fold(
                0,
                |acc, &x| {
                    if x > EPSILON {
                        acc + 1
                    } else {
                        acc
                    }
                },
            );

        if non_zeros + 1 == expected_non_zeros {
            (approx / (expected_non_zeros as f64)).round()
        } else {
            0.0
        }
    }

    pub fn regular(&self) -> bool {
        let b = (0..self.vertex_count).map(|i| self.laplacian[(i, i)])
            .skip_while(|&p| p == 0.0).next();

        if b.is_none() {
            return true;
        }

        let d = b.unwrap();

        (0..self.vertex_count).map(|i| self.laplacian[(i, i)])
            .all(|x| {
                (x == 0.0) || (x == d)
            })
    }

    #[inline]
    pub fn degree(&self, v: usize) -> usize {
        self.laplacian[(v, v)] as usize
    }

    #[inline]
    pub fn transfer(&self, other: &mut Self) {
        self.laplacian.clone_into(&mut other.laplacian);
        other.vertex_count = self.vertex_count;
        other.eigen_cache = None;
    }

    #[inline]
    pub fn convert_type<R: graph_type::GraphType>(self) -> LapGraph<R> {
        LapGraph::<R> {
            laplacian: self.laplacian,
            vertex_count: self.vertex_count,
            eigen_cache: self.eigen_cache,
            eigenvalue_cache: self.eigenvalue_cache,
            _p: PhantomData
        }
    }

    #[inline]
    pub fn erase_type(self) -> LapGraph<graph_type::Erased> {
        self.convert_type()
    }

    #[inline]
    pub fn isolated(&self, v: usize) -> bool {
        self.degree(v) == 0
    }
}

impl<T: graph_type::GraphType> Display for LapGraph<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let itr = (0..self.vertex_count)
            .flat_map(|a| (
                (a+1)..self.vertex_count
            )
                .map(move |b| (a, b))
                .filter(|(a, b)| self.laplacian[(*a, *b)] != 0.0)
            ).flat_map(|(a, b)| (0..(-self.laplacian[(a, b)].round() as usize)).map(move |_| (a, b)));

        write!(f, "{:?}\n", itr.collect::<Vec<(usize, usize)>>())
    }
}
