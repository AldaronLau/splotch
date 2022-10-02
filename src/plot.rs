// plot.rs
//
// Copyright (c) 2021  Douglas P Lau
// Copyright (c) 2022  Jeron A Lau
//
//! Plot types
use std::fmt;

use pointy::Pt;

use crate::{domain::Domain, page::Rect, scale::Numeric};

/// Private module for sealed Plot trait
mod sealed {
    use std::fmt;

    use crate::page::Rect;

    pub trait Plot {
        fn name(&self) -> &str;
        fn display(
            &self,
            f: &mut fmt::Formatter,
            num: usize,
            rect: Rect,
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
    domain: &'a Domain<Numeric, Numeric>,
    data: &'a [P],
}

/// Line plot
///
/// Data is drawn as a series of points connected by line segments.
pub struct Line<'a, P: Into<Pt<f32>> + Clone + 'a> {
    name: &'a str,
    domain: &'a Domain<Numeric, Numeric>,
    data: &'a [P],
}

/// Scatter plot
///
/// Data is drawn as unconnected points.
pub struct Scatter<'a, P: Into<Pt<f32>> + Clone + 'a> {
    name: &'a str,
    domain: &'a Domain<Numeric, Numeric>,
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
        rect: Rect,
    ) -> fmt::Result {
        write!(f, "<path class='plot-{} plot-area' d='", num)?;
        if let Some(pt) = self.data.first().cloned() {
            let x = self.domain.x_map(pt.into().x(), rect);
            let y = self.domain.y_map(0.0, rect);
            write!(f, "M{} {}", x, y)?;
        }
        for pt in self.data.iter().cloned() {
            let pt = pt.into();
            let x = self.domain.x_map(pt.x(), rect);
            let y = self.domain.y_map(pt.y(), rect);
            write!(f, " {} {}", x, y)?;
        }
        if let Some(pt) = self.data.last().cloned() {
            let x = self.domain.x_map(pt.into().x(), rect);
            let y = self.domain.y_map(0.0, rect);
            write!(f, " {} {}", x, y)?;
        }
        writeln!(f, "' />")
    }
}

impl<'a, P: Into<Pt<f32>> + Clone> Area<'a, P> {
    /// Create a new stacked area plot
    pub fn new(
        name: &'a str,
        domain: &'a Domain<Numeric, Numeric>,
        data: &'a [P],
    ) -> Self {
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
        rect: Rect,
    ) -> fmt::Result {
        write!(f, "<path class='plot-{} plot-line' d='", num)?;
        for (i, pt) in self.data.iter().cloned().enumerate() {
            let pt = pt.into();
            let x = self.domain.x_map(pt.x(), rect);
            let y = self.domain.y_map(pt.y(), rect);
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
    pub fn new(
        name: &'a str,
        domain: &'a Domain<Numeric, Numeric>,
        data: &'a [P],
    ) -> Self {
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
        rect: Rect,
    ) -> fmt::Result {
        write!(f, "<path class='plot-{} plot-scatter' d='", num)?;
        for (i, pt) in self.data.iter().cloned().enumerate() {
            let pt = pt.into();
            let x = self.domain.x_map(pt.x(), rect);
            let y = self.domain.y_map(pt.y(), rect);
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
    pub fn new(
        name: &'a str,
        domain: &'a Domain<Numeric, Numeric>,
        data: &'a [P],
    ) -> Self {
        Scatter { name, domain, data }
    }
}
