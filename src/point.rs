use crate::axis::Axis;
use crate::page::{Edge, Rect};
use crate::plot::{private::SealedPlot, Plot};
use crate::scale::{NumScale, Scale};
use std::fmt;

pub trait Point {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

pub struct PointPlot<'a, P>
where
    P: Point + 'a,
{
    data: &'a [P],
    x_domain: Option<NumScale>,
    y_domain: Option<NumScale>,
}

impl<'a, P> SealedPlot for PointPlot<'a, P>
where
    P: Point + 'a,
{
    fn display(&self, f: &mut fmt::Formatter, rect: Rect) -> fmt::Result {
        write!(f, "<path class='series-a' d='")?;
        let x_scale = self.x_scale();
        let y_scale = self.y_scale();
        for (i, pt) in self.data.iter().enumerate() {
            let x = rect.x as f32
                + f32::from(rect.width) * x_scale.proportion(pt.x());
            let y = rect.y as f32
                + f32::from(rect.height) * y_scale.proportion(pt.y());
            if i == 0 {
                write!(f, "M{} {}", x, y)?;
            } else {
                write!(f, " {} {}", x, y)?;
            }
        }
        writeln!(f, "' />")
    }
}

impl<'a, P> Plot for PointPlot<'a, P>
where
    P: Point + 'a,
{
}

impl<'a, P> From<PointPlot<'a, P>> for Box<dyn Plot + 'a>
where
    P: Point + 'a,
{
    fn from(plot: PointPlot<'a, P>) -> Self {
        Box::new(plot)
    }
}

impl<'a, P> PointPlot<'a, P>
where
    P: Point + 'a,
{
    pub fn new(data: &'a [P]) -> Self {
        PointPlot {
            data,
            x_domain: None,
            y_domain: None,
        }
    }

    pub fn x_domain<T>(mut self, data: &[T]) -> Self
    where
        T: Point,
    {
        self.x_domain = Some(NumScale::of_data(data, |pt| pt.x()));
        self
    }

    pub fn y_domain<T>(mut self, data: &[T]) -> Self
    where
        T: Point,
    {
        self.y_domain = Some(NumScale::of_data(data, |pt| pt.y()));
        self
    }

    fn x_scale(&self) -> NumScale {
        match &self.x_domain {
            Some(domain) => domain.clone(),
            None => NumScale::of_data(&self.data[..], |pt| pt.x()),
        }
    }

    pub fn x_axis(&self) -> Axis {
        let ticks = self.x_scale().ticks();
        Axis::new(Edge::Bottom, ticks)
    }

    fn y_scale(&self) -> NumScale {
        match &self.y_domain {
            Some(domain) => domain.clone().inverted(),
            None => NumScale::of_data(&self.data[..], |pt| pt.y()).inverted(),
        }
    }

    pub fn y_axis(&self) -> Axis {
        let ticks = self.y_scale().ticks();
        Axis::new(Edge::Left, ticks)
    }
}

impl Point for f32 {
    fn x(&self) -> f32 {
        *self
    }

    fn y(&self) -> f32 {
        *self
    }
}

impl Point for f64 {
    fn x(&self) -> f32 {
        *self as f32
    }

    fn y(&self) -> f32 {
        *self as f32
    }
}

impl Point for isize {
    fn x(&self) -> f32 {
        *self as f32
    }

    fn y(&self) -> f32 {
        *self as f32
    }
}

impl Point for i32 {
    fn x(&self) -> f32 {
        *self as f32
    }

    fn y(&self) -> f32 {
        *self as f32
    }
}

impl Point for (f32, f32) {
    fn x(&self) -> f32 {
        self.0
    }

    fn y(&self) -> f32 {
        self.1
    }
}

impl Point for (f64, f64) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

impl Point for (isize, isize) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

impl Point for (i8, i8) {
    fn x(&self) -> f32 {
        self.0.into()
    }

    fn y(&self) -> f32 {
        self.1.into()
    }
}

impl Point for (i16, i16) {
    fn x(&self) -> f32 {
        self.0.into()
    }

    fn y(&self) -> f32 {
        self.1.into()
    }
}

impl Point for (i32, i32) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

impl Point for (i64, i64) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

impl Point for (i128, i128) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [(45.0, 150.0), (90.0, 200.0)];
        let plot = PointPlot::new(&data).x_domain(&[0, 100]);
    }
}
