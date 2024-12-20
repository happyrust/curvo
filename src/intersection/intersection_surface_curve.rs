use argmin::core::{ArgminFloat, State};
use nalgebra::{
    allocator::Allocator, DefaultAllocator, DimName, DimNameDiff, DimNameSub, OPoint, U1,
};

use crate::{
    curve::NurbsCurve,
    misc::FloatingPoint,
    prelude::{BoundingBoxTraversal, CurveBoundingBoxTree, SurfaceBoundingBoxTree},
    surface::{NurbsSurface, UVDirection},
};

use super::{CurveIntersection, CurveIntersectionSolverOptions, Intersects};

impl<'a, T, D> Intersects<'a, &'a NurbsCurve<T, D>> for NurbsSurface<T, D>
where
    T: FloatingPoint + ArgminFloat,
    D: DimName + DimNameSub<U1>,
    DefaultAllocator: Allocator<D>,
    DefaultAllocator: Allocator<DimNameDiff<D, U1>>,
{
    type Output = anyhow::Result<Vec<CurveIntersection<OPoint<T, DimNameDiff<D, U1>>, T>>>;
    type Option = Option<CurveIntersectionSolverOptions<T>>;

    ///
    #[allow(clippy::type_complexity)]
    fn find_intersections(
        &'a self,
        other: &'a NurbsCurve<T, D>,
        option: Self::Option,
    ) -> Self::Output {
        let options = option.unwrap_or_default();

        let div = T::one() / T::from_usize(options.knot_domain_division).unwrap();
        let interval = self.knots_domain_interval();
        let ta = SurfaceBoundingBoxTree::new(
            self,
            UVDirection::U,
            Some((interval.0 * div, interval.1 * div)),
        );
        let tb = CurveBoundingBoxTree::new(other, Some(other.knots_domain_interval() * div));

        let traversed = BoundingBoxTraversal::try_traverse(ta, tb)?;
        let a_domain = self.knots_domain();
        let b_domain = other.knots_domain();
        todo!();

        /*
        let intersections = traversed
            .into_pairs_iter()
            .filter_map(|(a, b)| {
                let ca = a.surface_owned();
                let cb = b.curve_owned();

                let problem = CurveIntersectionProblem::new(&ca, &cb);

                // let inv = T::from_f64(0.5).unwrap();
                // let d0 = ca.knots_domain();
                // let d1 = cb.knots_domain();

                // Define initial parameter vector
                let init_param = Vector2::<T>::new(
                    ca.knots_domain().0,
                    cb.knots_domain().0,
                    // (d0.0 + d0.1) * inv,
                    // (d1.0 + d1.1) * inv,
                );

                // Set up solver
                let solver = CurveIntersectionBFGS::<T>::new()
                    .with_step_size_tolerance(options.step_size_tolerance)
                    .with_cost_tolerance(options.cost_tolerance);

                // Run solver
                let res = Executor::new(problem, solver)
                    .configure(|state| {
                        state
                            .param(init_param)
                            .inv_hessian(Matrix2::identity())
                            .max_iters(options.max_iters)
                    })
                    .run();

                match res {
                    Ok(r) => {
                        // println!("{}", r.state().get_termination_status());
                        r.state().get_best_param().and_then(|param| {
                            if (a_domain.0..=a_domain.1).contains(&param[0])
                                && (b_domain.0..=b_domain.1).contains(&param[1])
                            {
                                let p0 = self.point_at(param[0]);
                                let p1 = other.point_at(param[1]);
                                Some(CurveIntersection::new((p0, param[0]), (p1, param[1])))
                            } else {
                                None
                            }
                        })
                    }
                    Err(_e) => {
                        // println!("{}", e);
                        None
                    }
                }
            })
            .filter(|it| {
                // filter out intersections that are too close
                let p0 = &it.a().0;
                let p1 = &it.b().0;
                let d = (p0 - p1).norm();
                d < options.minimum_distance
            })
            .collect_vec();

        let sorted = intersections
            .into_iter()
            .sorted_by(|x, y| x.a().1.partial_cmp(&y.a().1).unwrap_or(Ordering::Equal))
            .collect_vec();

        // println!("sorted: {:?}", sorted.iter().map(|it| it.a()).collect_vec());

        // group near parameter results & extract the closest one in each group
        let parameter_minimum_distance = T::from_f64(1e-3).unwrap();
        let groups = sorted
            .into_iter()
            .map(|pt| vec![pt])
            .coalesce(|x, y| {
                let x0 = &x[x.len() - 1];
                let y0 = &y[y.len() - 1];
                let da = Float::abs(x0.a().1 - y0.a().1);
                let db = Float::abs(x0.b().1 - y0.b().1);
                if da < parameter_minimum_distance || db < parameter_minimum_distance {
                    // merge near parameter results
                    let group = [x, y].concat();
                    Ok(group)
                } else {
                    Err((x, y))
                }
            })
            .collect::<Vec<Vec<Intersects<OPoint<T, DimNameDiff<D, U1>>, T>>>>()
            .into_iter()
            .collect_vec();

        let pts = groups
            .into_iter()
            .filter_map(|group| match group.len() {
                1 => Some(group[0].clone()),
                _ => {
                    // find the closest intersection in the group
                    group
                        .iter()
                        .map(|it| {
                            let delta = &it.a().0 - &it.b().0;
                            let norm = delta.norm_squared();
                            (it, norm)
                        })
                        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
                        .map(|closest| closest.0.clone())
                }
            })
            .collect_vec();

        Ok(pts)
        */
    }
}
