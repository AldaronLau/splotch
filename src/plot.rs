// plot.rs
//
// Copyright (c) 2021  Douglas P Lau
// Copyright (c) 2022  Jeron A Lau
//
//! Plot types
use std::fmt;

use pointy::{BBox, Pt};

use crate::scale::Numeric;

/// Private module for sealed Plot trait
mod sealed {
    use std::fmt;

    use pointy::BBox;

    pub trait Plot {
        fn name(&self) -> &str;
        fn display(
            &self,
            f: &mut fmt::Formatter,
            num: usize,
            rect: BBox<f32>,
        ) -> fmt::Result;
    }
}

/// Plot for rendering data
///
/// This trait is *sealed* to hide details.
pub trait Plot: sealed::Plot {}

/// Stacked area plot
///
/// Data is drawn as filled-in areas, stacked vertically.
pub struct Area<'a, P: Into<Pt<f32>> + Clone + 'a> {
    name: &'a str,
    domain: &'a BBox<f32>,
    data: &'a [P],
}

/// Line plot
///
/// Data is drawn as a series of points connected by line segments.
pub struct Line<'a, P: Into<Pt<f32>> + Clone + 'a> {
    name: &'a str,
    domain: &'a BBox<f32>,
    data: &'a [P],
}

/// Scatter plot
///
/// Data is drawn as unconnected points.
pub struct Scatter<'a, P: Into<Pt<f32>> + Clone + 'a> {
    name: &'a str,
    domain: &'a BBox<f32>,
    data: &'a [P],
}

impl<'a, P> Plot for Area<'a, P> where P: Into<Pt<f32>> + Clone {}

impl<'a, P: Into<Pt<f32>> + Clone> sealed::Plot for Area<'a, P> {
    fn name(&self) -> &str {
        self.name
    }

    fn display(
        &self,
        f: &mut fmt::Formatter,
        num: usize,
        rect: BBox<f32>,
    ) -> fmt::Result {
        write!(f, "<path class='plot-{} plot-area' d='", num)?;
        if let Some(pt) = self.data.first().cloned() {
            let x = x_map(self.domain, pt.into().x(), rect);
            let y = y_map(self.domain, 0.0, rect);
            write!(f, "M{} {}", x, y)?;
        }
        for pt in self.data.iter().cloned() {
            let pt = pt.into();
            let x = x_map(self.domain, pt.x(), rect);
            let y = y_map(self.domain, pt.y(), rect);
            write!(f, " {} {}", x, y)?;
        }
        if let Some(pt) = self.data.last().cloned() {
            let x = x_map(self.domain, pt.into().x(), rect);
            let y = y_map(self.domain, 0.0, rect);
            write!(f, " {} {}", x, y)?;
        }
        writeln!(f, "' />")
    }
}

impl<'a, P: Into<Pt<f32>> + Clone> Area<'a, P> {
    /// Create a new stacked area plot
    pub fn new(name: &'a str, domain: &'a BBox<f32>, data: &'a [P]) -> Self {
        Area { name, domain, data }
    }
}

impl<'a, P> Plot for Line<'a, P> where P: Into<Pt<f32>> + Clone {}

impl<'a, P: Into<Pt<f32>> + Clone> sealed::Plot for Line<'a, P> {
    fn name(&self) -> &str {
        self.name
    }

    fn display(
        &self,
        f: &mut fmt::Formatter,
        num: usize,
        rect: BBox<f32>,
    ) -> fmt::Result {
        write!(f, "<path class='plot-{} plot-line' d='", num)?;
        for (i, pt) in self.data.iter().cloned().enumerate() {
            let pt = pt.into();
            let x = x_map(self.domain, pt.x(), rect);
            let y = y_map(self.domain, pt.y(), rect);
            if i == 0 {
                write!(f, "M{} {}", x, y)?;
            } else {
                write!(f, " {} {}", x, y)?;
            }
        }
        writeln!(f, "'/>")
    }
}

impl<'a, P: Into<Pt<f32>> + Clone> Line<'a, P> {
    /// Create a new line plot
    pub fn new(name: &'a str, domain: &'a BBox<f32>, data: &'a [P]) -> Self {
        Line { name, domain, data }
    }
}

impl<'a, P> Plot for Scatter<'a, P> where P: Into<Pt<f32>> + Clone {}

impl<'a, P: Into<Pt<f32>> + Clone> sealed::Plot for Scatter<'a, P> {
    fn name(&self) -> &str {
        self.name
    }

    fn display(
        &self,
        f: &mut fmt::Formatter,
        num: usize,
        rect: BBox<f32>,
    ) -> fmt::Result {
        write!(f, "<path class='plot-{} plot-scatter' d='", num)?;
        for (i, pt) in self.data.iter().cloned().enumerate() {
            let pt = pt.into();
            let x = x_map(self.domain, pt.x(), rect);
            let y = y_map(self.domain, pt.y(), rect);
            if i == 0 {
                write!(f, "M{} {}", x, y)?;
            } else {
                write!(f, " {} {}", x, y)?;
            }
        }
        writeln!(f, "' />")
    }
}

impl<'a, P: Into<Pt<f32>> + Clone> Scatter<'a, P> {
    /// Create a new scatter plot
    pub fn new(name: &'a str, domain: &'a BBox<f32>, data: &'a [P]) -> Self {
        Scatter { name, domain, data }
    }
}

/// Normalize an `X` value
fn x_norm(domain: BBox<f32>, x: f32) -> f32 {
    let x_scale = Numeric::from_data(domain, |pt| pt.x());
    x_scale.normalize(x)
}

/// Normalize a `Y` value
fn y_norm(domain: BBox<f32>, y: f32) -> f32 {
    let y_scale = Numeric::from_data(domain, |pt| pt.y());
    y_scale.inverted().normalize(y)
}

/// Map an `X` value to a rectangle
pub(crate) fn x_map(domain: &BBox<f32>, x: f32, rect: BBox<f32>) -> i32 {
    let rx = rect.x_min();
    let rw = rect.x_span();
    let mx = rx + rw * x_norm(*domain, x);
    mx.round() as i32
}

/// Map a `Y` value to a rectangle
pub(crate) fn y_map(domain: &BBox<f32>, y: f32, rect: BBox<f32>) -> i32 {
    let ry = rect.y_min();
    let rh = rect.y_span();
    let my = ry + rh * y_norm(*domain, y);
    my.round() as i32
}
