// axis.rs
//
// Copyright (c) 2021  Douglas P Lau
// Copyright (c) 2022  Jeron A Lau
//
//! Axis for charts

// FIXME
#![allow(dead_code)]

use std::{fmt, fmt::Write};

use pointy::BBox;

use crate::{
    page::Edge,
    scale::Numeric,
    text::{Anchor, Label, Text, Tick},
};

/// Private module for sealed Axis trait
mod sealed {
    use std::{fmt, fmt::Write};

    use pointy::BBox;

    pub trait Axis {
        fn split(&self, area: &mut BBox<f32>) -> BBox<f32>;
        fn display(
            &self,
            f: &mut dyn Write,
            rect: BBox<f32>,
            area: BBox<f32>,
        ) -> fmt::Result;
        fn display_grid(
            &self,
            f: &mut dyn Write,
            area: BBox<f32>,
        ) -> fmt::Result;
    }
}

/// Axis for drawing labels on a `Chart`
///
/// This trait is *sealed* to hide details.  There are two implementors:
/// - `axis::Horizontal`
/// - `axis::Vertical`
pub trait Axis: sealed::Axis {}

/// Horizontal `X` axis
#[derive(Debug, PartialEq)]
pub struct Horizontal {
    edge: Edge,
    ticks: Vec<Tick>,
    name: Option<String>,
    label: Label,
}

/// Vertical `Y` axis
#[derive(Debug, PartialEq)]
pub struct Vertical {
    edge: Edge,
    ticks: Vec<Tick>,
    name: Option<String>,
    label: Label,
}

impl sealed::Axis for Horizontal {
    fn split(&self, area: &mut BBox<f32>) -> BBox<f32> {
        self.edge.split(area, f32::from(self.space()))
    }

    fn display(
        &self,
        f: &mut dyn Write,
        mut rect: BBox<f32>,
        area: BBox<f32>,
    ) -> fmt::Result {
        intersect_horiz(&mut rect, &area);
        if let Some(name) = &self.name {
            let r = self.edge.split(&mut rect, f32::from(self.space() / 2));
            let text =
                Text::new(self.edge).with_rect(r).with_class_name("axis");
            text.display(f)?;
            writeln!(f, "{}", name)?;
            text.display_done(f)?;
        }
        self.display_tick_lines(f, rect)?;
        self.display_tick_labels(f, rect)
    }

    fn display_grid(&self, f: &mut dyn Write, area: BBox<f32>) -> fmt::Result {
        write!(f, "<path class='grid-x' d='")?;
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, area, 0.0);
            write!(f, "M{} {}v{}", x, area.y_min(), area.y_span())?;
        }
        writeln!(f, "'/>")
    }
}

impl Axis for Horizontal {}

impl Horizontal {
    /// Create a new horizontal axis
    pub fn new(domain: BBox<f32>) -> Self {
        let x_scale = Numeric::from_data(domain, |pt| pt.x());

        Self {
            edge: Edge::Bottom,
            ticks: x_scale.ticks(),
            name: None,
            label: Label::new(),
        }
    }

    /// Set the name of the axis
    pub fn with_name<N>(mut self, name: N) -> Self
    where
        N: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    /// Attach to the top of a `Chart`
    ///
    /// By default, a `Horizontal` axis is attached to the bottom of a `Chart`.
    pub fn on_top(mut self) -> Self {
        self.edge = Edge::Top;
        self
    }

    fn space(&self) -> u16 {
        match self.name {
            Some(_) => 160,
            None => 80,
        }
    }

    fn display_tick_lines(
        &self,
        f: &mut dyn Write,
        rect: BBox<f32>,
    ) -> fmt::Result {
        let x = rect.x_min();
        let (y, height) = match self.edge {
            Edge::Top => (rect.y_max(), Tick::LEN),
            Edge::Bottom => (rect.y_min(), -Tick::LEN),
            _ => unreachable!(),
        };
        write!(
            f,
            "<path class='axis-line' d='M{} {}h{}",
            x,
            y,
            rect.x_span()
        )?;
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, rect, Tick::LEN as f32) as i32;
            let y = tick.y(self.edge, rect, Tick::LEN as f32) as i32;
            let y0 = y.min(y + height);
            let h = y.max(y + height) - y0;
            write!(f, "M{} {}v{}", x, y0, h)?;
        }
        writeln!(f, "'/>")
    }

    fn display_tick_labels(
        &self,
        f: &mut dyn Write,
        rect: BBox<f32>,
    ) -> fmt::Result {
        let text = Text::new(Edge::Top).with_class_name("tick");
        text.display(f)?;
        for tick in &self.ticks {
            tick.tspan(self.edge, rect).display(f)?;
        }
        text.display_done(f)
    }
}

impl sealed::Axis for Vertical {
    fn split(&self, area: &mut BBox<f32>) -> BBox<f32> {
        self.edge.split(area, self.space().into())
    }

    fn display(
        &self,
        f: &mut dyn Write,
        mut rect: BBox<f32>,
        area: BBox<f32>,
    ) -> fmt::Result {
        intersect_vert(&mut rect, &area);
        if let Some(name) = &self.name {
            let r = self.edge.split(&mut rect, f32::from(self.space() / 2));
            let text =
                Text::new(self.edge).with_rect(r).with_class_name("axis");
            text.display(f)?;
            writeln!(f, "{}", name)?;
            text.display_done(f)?;
        }
        self.display_tick_lines(f, rect)?;
        self.display_tick_labels(f, rect)
    }

    fn display_grid(&self, f: &mut dyn Write, area: BBox<f32>) -> fmt::Result {
        write!(f, "<path class='grid-y' d='")?;
        for tick in self.ticks.iter() {
            let y = tick.y(self.edge, area, 0.0);
            write!(f, "M{} {}h{}", area.x_min(), y, area.x_span())?;
        }
        writeln!(f, "'/>")
    }
}

impl Axis for Vertical {}

impl Vertical {
    /// Create a new vertical axis
    pub fn new(domain: BBox<f32>) -> Self {
        let y_scale = Numeric::from_data(domain, |pt| pt.y());

        Self {
            edge: Edge::Left,
            ticks: y_scale.inverted().ticks(),
            name: None,
            label: Label::new(),
        }
    }

    /// Set the name of the axis
    pub fn with_name<N>(mut self, name: N) -> Self
    where
        N: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    /// Attach to the right side of a `Chart`
    ///
    /// By default, a `Vertical` axis is attached to the left side of a `Chart`.
    pub fn on_right(mut self) -> Self {
        self.edge = Edge::Right;
        self
    }

    fn space(&self) -> u16 {
        match self.name {
            Some(_) => 160,
            None => 80,
        }
    }

    fn display_tick_lines(
        &self,
        f: &mut dyn Write,
        rect: BBox<f32>,
    ) -> fmt::Result {
        let (x, width) = match self.edge {
            Edge::Left => (rect.x_max(), Tick::LEN),
            Edge::Right => (rect.x_min(), -Tick::LEN),
            _ => unreachable!(),
        };
        write!(f, "<path class='axis-line'")?;
        write!(f, " d='M{} {}v{}", x, rect.y_min(), rect.y_span())?;
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, rect, Tick::LEN as f32) as i32;
            let y = tick.y(self.edge, rect, Tick::LEN as f32) as i32;
            let x0 = x.min(x + width);
            let w = x.max(x + width) - x0;
            write!(f, " M{} {}h{}", x0, y, w)?;
        }
        writeln!(f, "'/>")
    }

    fn display_tick_labels(
        &self,
        f: &mut dyn Write,
        rect: BBox<f32>,
    ) -> fmt::Result {
        let anchor = match self.edge {
            Edge::Left => Anchor::End,
            Edge::Right => Anchor::Start,
            _ => unreachable!(),
        };
        let text = Text::new(Edge::Top)
            .with_anchor(anchor)
            .with_class_name("tick");
        text.display(f)?;
        for tick in &self.ticks {
            tick.tspan(self.edge, rect).display(f)?;
        }
        text.display_done(f)
    }
}

fn intersect_horiz(this: &mut BBox<f32>, rhs: &BBox<f32>) {
    *this = BBox::new([
        (this.x_min().max(rhs.x_min()), this.y_min()),
        (this.x_max().min(rhs.x_max()), this.y_max()),
    ]);
}

fn intersect_vert(this: &mut BBox<f32>, rhs: &BBox<f32>) {
    *this = BBox::new([
        (this.x_min(), this.y_min().max(rhs.y_min())),
        (this.x_max(), this.y_max().min(rhs.y_max())),
    ]);
}
