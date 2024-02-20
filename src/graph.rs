use std::any::type_name;
use std::fmt::{Display, Formatter};
use nalgebra::{Const, DMatrix, Dyn, Matrix, max, OMatrix, OVector, SMatrix, SymmetricEigen, ToTypenum};


#[derive(Debug)]
pub struct LapGraph<const resizable: bool> {
    // f32 is actually quite useful here, despite its scariness
    laplacian: OMatrix<f32, Dyn, Dyn>,
    vertex_count: usize
}

// TODO: Resizing doesn't work
impl<const Resizable: bool> LapGraph<Resizable> {
    pub fn empty(vertex_count: usize) -> Self {
        Self {
            laplacian: DMatrix::<f32>::zeros(vertex_count, vertex_count),
            vertex_count
        }
    }

    pub fn add_edge(&mut self, i: usize, j: usize) {
        debug_assert!(i != j, "LapGraph::<{}>::add_edge does not support self loops", self.vertex_count);

        if Resizable { // Nonexistent if statement
            if i >= self.vertex_count || j >= self.vertex_count {
                if i > j {
                    self.laplacian.resize_mut(i, i, 0.0);
                    self.vertex_count = i + 1;
                }else {
                    self.laplacian.resize_mut(j, j, 0.0);
                    self.vertex_count = j + 1;
                }
            }
        }else {
            debug_assert!(i < self.vertex_count && j < self.vertex_count, "LabGraph::<{}>::add_edge indices must lie in [0, {})", self.vertex_count, self.vertex_count);
        }

        if self.laplacian[(i, j)] == 0.0 {
            self.laplacian[(i, j)] = -1.0;
            self.laplacian[(j, i)] = -1.0;
            self.laplacian[(i, i)] += 1.0;
            self.laplacian[(j, j)] += 1.0;
        }
    }

    pub fn remove_edge(&mut self, i: usize, j: usize) {
        debug_assert!(i != j, "LapGraph::<{}>::remove_edge does not support self loops", self.vertex_count);

        if Resizable { // Nonexistent if statement
            // In this case, we are actually removing a nonexistent edge. It is in reality a resize for a NOP.
            if i >= self.vertex_count || j >= self.vertex_count {
                println!("X: {:?}", self.laplacian);
                if i > j {
                    self.laplacian.resize_mut(i, i, 0.0);
                    self.vertex_count = i + 1;
                }else {
                    self.laplacian.resize_mut(j, j, 0.0);
                    self.vertex_count = j + 1;
                }
            }
        }else {
            debug_assert!(i < self.vertex_count && j < self.vertex_count, "LabGraph::<{}>::remove_edge indices must lie in [0, {})", self.vertex_count, self.vertex_count);
        }

        if self.laplacian[(i, j)] == -1.0 {
            self.laplacian[(i, j)] = 0.0;
            self.laplacian[(j, i)] = 0.0;
            self.laplacian[(i, i)] -= 1.0;
            self.laplacian[(j, j)] -= 1.0;
        }
    }

    pub fn eigenvalues(&mut self) {
    }

    pub fn eigenvectors(&mut self) {
    }
}

impl<const Resizable: bool> Display for LapGraph<Resizable> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.laplacian)
    }
}