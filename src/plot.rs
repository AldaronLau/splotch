// plot.rs
//
// Copyright (c) 2021  Douglas P Lau
// Copyright (c) 2022  Jeron A Lau
//
//! Plot types

use std::{fmt, fmt::Write};

use pointy::{BBox, Pt};

use crate::scale::Numeric;

#[derive(Copy, Clone, Debug)]
pub(crate) enum PlotKind {
    Area,
    Line,
    Scatter,
}

/// Generic plot
///
/// The type of plot that's rendered is determined at a later step.
pub struct Plot<'a> {
    name: &'a str,
    domain: &'a BBox<f32>,
    data: &'a mut dyn Iterator<Item = Pt<f32>>,
}

impl<'a> Plot<'a> {
    pub fn new(
        name: &'a str,
        domain: &'a BBox<f32>,
        data: &'a mut dyn Iterator<Item = Pt<f32>>,
    ) -> Self {
        Self { name, domain, data }
    }

    fn display_area(
        &mut self,
        f: &mut dyn Write,
        num: usize,
        rect: BBox<f32>,
    ) -> fmt::Result {
        let mut iter = self.data.peekable();

        write!(f, "<path class='plot-{num} plot-area' d='")?;

        if let Some(pt) = iter.peek() {
            let x = x_map(self.domain, pt.x(), rect);
            let y = y_map(self.domain, 0.0, rect);
            write!(f, "M{x} {y}")?;
        }

        while let Some(pt) = iter.next() {
            let x = x_map(self.domain, pt.x(), rect);
            let y = y_map(self.domain, pt.y(), rect);
            write!(f, " {x} {y}")?;

            if iter.peek().is_none() {
                let x = x_map(self.domain, pt.x(), rect);
                let y = y_map(self.domain, 0.0, rect);
                write!(f, " {x} {y}")?;
            }
        }

        writeln!(f, "' />")
    }

    fn display_line(
        &mut self,
        f: &mut dyn Write,
        num: usize,
        rect: BBox<f32>,
    ) -> fmt::Result {
        write!(f, "<path class='plot-{num} plot-line' d='")?;

        for (i, pt) in self.data.enumerate() {
            let x = x_map(self.domain, pt.x(), rect);
            let y = y_map(self.domain, pt.y(), rect);

            if i == 0 {
                write!(f, "M{x} {y}")?;
            } else {
                write!(f, " {x} {y}")?;
            }
        }
        writeln!(f, "'/>")
    }

    fn display_scatter(
        &mut self,
        f: &mut dyn Write,
        num: usize,
        rect: BBox<f32>,
    ) -> fmt::Result {
        write!(f, "<path class='plot-{num} plot-scatter' d='")?;

        for (i, pt) in self.data.enumerate() {
            let x = x_map(self.domain, pt.x(), rect);
            let y = y_map(self.domain, pt.y(), rect);

            if i == 0 {
                write!(f, "M{x} {y}")?;
            } else {
                write!(f, " {x} {y}")?;
            }
        }
        writeln!(f, "' />")
    }

    pub(crate) fn name(&self) -> &'a str {
        self.name
    }

    pub(crate) fn display(
        &mut self,
        f: &mut dyn Write,
        num: usize,
        rect: BBox<f32>,
        kind: PlotKind,
    ) -> fmt::Result {
        use PlotKind::*;

        match kind {
            Area => self.display_area(f, num, rect),
            Line => self.display_line(f, num, rect),
            Scatter => self.display_scatter(f, num, rect),
        }
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
