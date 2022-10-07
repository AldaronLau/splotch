// text.rs
//
// Copyright (c) 2021  Douglas P Lau
// Copyright (c) 2022  Jeron A Lau
//

// FIXME
#![allow(dead_code)]

use std::{fmt, fmt::Write};

use pointy::BBox;

use crate::page::Edge;

/// Text label point
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LabelPoint {
    /// Minimum point (start of bar/column)
    Minimum,
    /// Center point
    Center,
    /// Maximum point (end of bar/column)
    Maximum,
}

/// Vertical offset relative to point
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VerticalOffset {
    /// Label below point
    Below,
    /// Label at point
    At,
    /// Label above point
    Above,
}

/// Text anchor
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Anchor {
    /// Anchor at start of text
    Start,
    /// Anchor at middle of text
    Middle,
    /// Anchor at end of text
    End,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Label {
    point: LabelPoint,
    offset: VerticalOffset,
    anchor: Anchor,
    rounding_precision: Option<usize>,
}

pub struct Text<'a> {
    edge: Edge,
    anchor: Anchor,
    rect: Option<BBox<f32>>,
    dy: Option<f32>,
    class_name: Option<&'a str>,
}

pub struct Tspan<'a> {
    text: &'a str,
    x: Option<i32>,
    y: Option<i32>,
    dy: Option<f32>,
}

/// Tick marks for axis labels
#[derive(Debug, PartialEq)]
pub struct Tick {
    value: f32,
    text: String,
}

impl Anchor {
    pub(crate) fn display(&self, f: &mut dyn Write) -> fmt::Result {
        match self {
            Anchor::Start => write!(f, " text-anchor='start'"),
            Anchor::Middle => write!(f, " text-anchor='middle'"),
            Anchor::End => write!(f, " text-anchor='end'"),
        }
    }
}

impl Default for Label {
    fn default() -> Self {
        Label {
            point: LabelPoint::Center,
            offset: VerticalOffset::At,
            anchor: Anchor::Middle,
            rounding_precision: None,
        }
    }
}

impl Label {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label_point(&self) -> LabelPoint {
        self.point
    }

    pub fn vertical_offset(&self) -> f32 {
        match self.offset {
            VerticalOffset::Above => -1.0,
            VerticalOffset::At => 0.0,
            VerticalOffset::Below => 1.0,
        }
    }

    pub fn minimum(mut self) -> Self {
        self.point = LabelPoint::Minimum;
        self
    }

    pub fn maximum(mut self) -> Self {
        self.point = LabelPoint::Maximum;
        self
    }

    pub fn above(mut self) -> Self {
        self.offset = VerticalOffset::Above;
        self
    }

    pub fn below(mut self) -> Self {
        self.offset = VerticalOffset::Below;
        self
    }

    pub fn start(mut self) -> Self {
        self.anchor = Anchor::Start;
        self
    }

    pub fn end(mut self) -> Self {
        self.anchor = Anchor::End;
        self
    }

    pub fn rounded(&self, value: f32) -> String {
        match self.rounding_precision {
            None => value.to_string(),
            Some(digits) => format!("{:.1$}", value, digits),
        }
    }
}

impl<'a> Text<'a> {
    pub fn new(edge: Edge) -> Self {
        Text {
            edge,
            anchor: Anchor::Middle,
            rect: None,
            dy: None,
            class_name: None,
        }
    }

    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn with_dy(mut self, dy: f32) -> Self {
        self.dy = Some(dy);
        self
    }

    pub fn with_rect(mut self, rect: BBox<f32>) -> Self {
        self.rect = Some(rect);
        self
    }

    pub fn with_class_name(mut self, class_name: &'a str) -> Self {
        self.class_name = Some(class_name);
        self
    }

    pub fn display(&self, f: &mut dyn Write) -> fmt::Result {
        write!(f, "<text")?;
        if let Some(class_name) = self.class_name {
            write!(f, " class='{}'", class_name)?;
        }
        if let Some(rect) = self.rect {
            self.transform(f, rect)?;
        }
        if let Some(dy) = self.dy {
            write!(f, " dy='{}em'", dy)?;
        }
        self.anchor.display(f)?;
        writeln!(f, ">")
    }

    pub fn display_done(&self, f: &mut dyn Write) -> fmt::Result {
        writeln!(f, "</text>")
    }

    fn transform(&self, f: &mut dyn Write, rect: BBox<f32>) -> fmt::Result {
        let x = match (self.edge, self.anchor) {
            (Edge::Top, Anchor::Start) | (Edge::Bottom, Anchor::Start) => {
                rect.x_min() as i32
            }
            (Edge::Top, Anchor::End) | (Edge::Bottom, Anchor::End) => {
                rect.x_max() as i32
            }
            _ => rect.x_min() as i32 + (rect.x_span() as i32) / 2,
        };
        let y = match (self.edge, self.anchor) {
            (Edge::Left, Anchor::End) | (Edge::Right, Anchor::Start) => {
                rect.y_min() as i32
            }
            (Edge::Left, Anchor::Start) | (Edge::Right, Anchor::End) => {
                rect.y_max() as i32
            }
            _ => rect.y_min() as i32 + (rect.y_span() as i32) / 2,
        };
        write!(f, " transform='translate({} {})", x, y)?;
        match self.edge {
            Edge::Left => write!(f, " rotate(-90)")?,
            Edge::Right => write!(f, " rotate(90)")?,
            _ => (),
        }
        write!(f, "'")
    }
}

impl<'a> Tspan<'a> {
    pub fn new(text: &'a str) -> Self {
        Tspan {
            text,
            x: None,
            y: None,
            dy: None,
        }
    }

    pub fn x(mut self, x: i32) -> Self {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: i32) -> Self {
        self.y = Some(y);
        self
    }

    pub fn dy(mut self, dy: f32) -> Self {
        self.dy = Some(dy);
        self
    }

    pub fn display(&self, f: &mut dyn Write) -> fmt::Result {
        write!(f, "<tspan")?;
        if let Some(x) = self.x {
            write!(f, " x='{}'", x)?;
        }
        if let Some(y) = self.y {
            write!(f, " y='{}'", y)?;
        }
        if let Some(dy) = self.dy {
            write!(f, " dy='{}em'", dy)?;
        }
        write!(f, ">{}", &self.text)?;
        writeln!(f, "</tspan>")
    }
}

impl Tick {
    pub const HLEN: i32 = Tick::LEN + 8;
    pub const LEN: i32 = 20;
    pub const VLEN: i32 = Tick::LEN * 2;

    pub fn new<T>(value: f32, text: T) -> Self
    where
        T: Into<String>,
    {
        let text = text.into();
        Tick { value, text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn x(&self, edge: Edge, rect: BBox<f32>, len: f32) -> f32 {
        match edge {
            Edge::Left => rect.x_max() - len,
            Edge::Right => rect.x_min() + len,
            _ => self.value * rect.x_span() + rect.x_min(),
        }
    }

    pub fn y(&self, edge: Edge, rect: BBox<f32>, len: f32) -> f32 {
        match edge {
            Edge::Top => rect.y_max() - len,
            Edge::Bottom => rect.y_min() + len,
            _ => self.value * rect.y_span() + rect.y_min(),
        }
    }

    pub fn tspan(&self, edge: Edge, rect: BBox<f32>) -> Tspan {
        let x = self.x(edge, rect, Tick::HLEN as f32) as i32;
        let y = self.y(edge, rect, Tick::VLEN as f32) as i32;
        Tspan::new(self.text()).x(x).y(y).dy(0.33)
    }
}
