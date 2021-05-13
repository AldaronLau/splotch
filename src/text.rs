use crate::page::{Edge, Rect};
use std::fmt;

/// Text label point
#[derive(Copy, Clone, Debug)]
pub enum LabelPoint {
    /// Minimum point (start of bar/column)
    Minimum,
    /// Center point
    Center,
    /// Maximum point (end of bar/column)
    Maximum,
}

/// Vertical offset relative to point
#[derive(Copy, Clone, Debug)]
pub enum VerticalOffset {
    /// Label below point
    Below,
    /// Label at point
    At,
    /// Label above point
    Above,
}

/// Text anchor
#[derive(Copy, Clone, Debug)]
pub enum Anchor {
    /// Anchor at start of text
    Start,
    /// Anchor at middle of text
    Middle,
    /// Anchor at end of text
    End,
}

#[derive(Clone, Debug)]
pub struct Label {
    point: LabelPoint,
    offset: VerticalOffset,
    pub anchor: Anchor,
    pub rounding_precision: Option<usize>,
}

pub struct Text<'a> {
    edge: Edge,
    anchor: Anchor,
    rect: Option<Rect>,
    dy: Option<f32>,
    class_name: Option<&'a str>,
}

pub struct Tspan<'a> {
    text: &'a str,
    x: Option<i32>,
    y: Option<i32>,
    dy: Option<f32>,
}

impl fmt::Display for Anchor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

    pub fn point(&self) -> LabelPoint {
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
    pub fn new(edge: Edge, anchor: Anchor) -> Self {
        Text {
            edge,
            anchor,
            rect: None,
            dy: None,
            class_name: None,
        }
    }

    pub fn dy(mut self, dy: f32) -> Self {
        self.dy = Some(dy);
        self
    }

    pub fn rect(mut self, rect: Rect) -> Self {
        self.rect = Some(rect);
        self
    }

    pub fn class_name(mut self, class_name: &'a str) -> Self {
        self.class_name = Some(class_name);
        self
    }

    pub fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<text")?;
        if let Some(class_name) = self.class_name {
            write!(f, " class='{}'", class_name)?;
        }
        if let Some(rect) = &self.rect {
            self.transform(f, rect)?;
        }
        if let Some(dy) = self.dy {
            write!(f, " dy='{}em'", dy)?;
        }
        writeln!(f, "{}>", self.anchor)
    }

    pub fn display_done(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "</text>")
    }

    fn transform(&self, f: &mut fmt::Formatter, rect: &Rect) -> fmt::Result {
        let x = match (self.edge, self.anchor) {
            (Edge::Top, Anchor::Start) | (Edge::Bottom, Anchor::Start) => {
                rect.x
            }
            (Edge::Top, Anchor::End) | (Edge::Bottom, Anchor::End) => {
                rect.x + i32::from(rect.width)
            }
            _ => rect.x + i32::from(rect.width) / 2,
        };
        let y = match (self.edge, self.anchor) {
            (Edge::Left, Anchor::End) | (Edge::Right, Anchor::Start) => rect.y,
            (Edge::Left, Anchor::Start) | (Edge::Right, Anchor::End) => {
                rect.y + i32::from(rect.height)
            }
            _ => rect.y + i32::from(rect.height) / 2,
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

    pub fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
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