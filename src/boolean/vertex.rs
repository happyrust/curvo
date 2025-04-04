use std::borrow::{Borrow, Cow};

use nalgebra::{Point2, U3};

use crate::{curve::NurbsCurve, misc::FloatingPoint};

use super::has_parameter::HasParameter;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Vertex<'a, T: FloatingPoint> {
    curve: Cow<'a, NurbsCurve<T, U3>>,
    position: Point2<T>,
    parameter: T,
}

impl<'a, T: FloatingPoint> Vertex<'a, T> {
    pub fn new(curve: &'a NurbsCurve<T, U3>, position: Point2<T>, parameter: T) -> Self {
        Self {
            curve: Cow::Borrowed(curve),
            position,
            parameter,
        }
    }

    #[allow(unused)]
    pub fn curve(&self) -> &NurbsCurve<T, U3> {
        self.curve.borrow()
    }

    pub fn position(&self) -> &Point2<T> {
        &self.position
    }
}

impl<T: FloatingPoint> HasParameter<T> for Vertex<'_, T> {
    fn parameter(&self) -> T {
        self.parameter
    }
}
