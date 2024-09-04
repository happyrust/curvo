use super::has_intersection_parameter::HasIntersectionParameter;

/// A struct representing the intersection of two curves.
#[derive(Debug, Clone)]
pub struct CurveIntersection<P, T> {
    /// The point & parameter of the first curve at the intersection.
    a: (P, T),
    /// The point & parameter of the second curve at the intersection.
    b: (P, T),
}

impl<P, T> CurveIntersection<P, T> {
    pub fn new(a: (P, T), b: (P, T)) -> Self {
        Self { a, b }
    }

    pub fn a(&self) -> &(P, T) {
        &self.a
    }

    pub fn b(&self) -> &(P, T) {
        &self.b
    }

    pub fn as_tuple(self) -> ((P, T), (P, T)) {
        (self.a, self.b)
    }
}

impl<P, T: Clone> HasIntersectionParameter<T> for CurveIntersection<P, T> {
    fn a_parameter(&self) -> T {
        self.a.1.clone()
    }

    fn b_parameter(&self) -> T {
        self.b.1.clone()
    }
}
