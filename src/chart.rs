// chart.rs
//
// Copyright (c) 2021  Douglas P Lau
// Copyright (c) 2022  Jeron A Lau
//
use std::{fmt, fmt::Write};

use pointy::{BBox, Pt};

use crate::{
    axis::Axis,
    page::{AspectRatio, Edge},
    plot::{Plot, PlotKind},
    text::{Anchor, Text},
};

/// Marker shapes
const MARKERS: &[&str] = &[
    "<circle r='1'/>",
    "<rect x='-1' y='-1' width='2' height='2'/>",
    "<path d='M0 -1 1 1 -1 1z'/>",
    "<path d='M1 0 -1 1 -1 -1z'/>",
    "<path d='M0 1 -1 -1 1 -1z'/>",
    "<path d='M-1 0 1 -1 1 1z'/>",
    "<path d='M0 -1 1 0 0 1 -1 0z'/>",
    "<path d='M-1 -1 0 -0.5 1 -1 0.5 0 1 1 0 0.5 -1 1 -0.5 0z'/>",
];

/// Chart title
pub struct Title {
    text: String,
    anchor: Anchor,
    edge: Edge,
}

/// Chart for plotting data
///
/// Multiple `Plot`s can be rendered in a single Chart, even with unrelated
/// domains and axes.
pub struct Chart<'a> {
    aspect_ratio: AspectRatio,
    titles: Vec<Title>,
    axes: Vec<Box<dyn Axis + 'a>>,
    plots: Vec<(PlotKind, Plot<'a>)>,
}

impl<T: Into<String>> From<T> for Title {
    fn from(text: T) -> Self {
        Title::new(text.into())
    }
}

impl Title {
    /// Create a new title
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            anchor: Anchor::Middle,
            edge: Edge::Top,
        }
    }

    /// Anchor title text at start
    pub fn at_start(mut self) -> Self {
        self.anchor = Anchor::Start;
        self
    }

    /// Anchor title text at end
    pub fn at_end(mut self) -> Self {
        self.anchor = Anchor::End;
        self
    }

    /// Put title on bottom of chart
    pub fn on_bottom(mut self) -> Self {
        self.edge = Edge::Bottom;
        self
    }

    /// Put title on left side of chart
    pub fn on_left(mut self) -> Self {
        self.edge = Edge::Left;
        self
    }

    /// Put title on right side of chart
    pub fn on_right(mut self) -> Self {
        self.edge = Edge::Right;
        self
    }

    fn display(&self, f: &mut dyn Write, rect: BBox<f32>) -> fmt::Result {
        let text = Text::new(self.edge)
            .with_rect(rect)
            .with_anchor(self.anchor)
            .with_class_name("title");
        text.display(f)?;
        writeln!(f, "{}", self.text)?;
        text.display_done(f)
    }
}

impl<'a> Default for Chart<'a> {
    fn default() -> Self {
        Self {
            aspect_ratio: AspectRatio::Landscape,
            titles: vec![],
            axes: vec![],
            plots: vec![],
        }
    }
}

impl<'a> Chart<'a> {
    /// Adjust the aspect ratio
    pub fn with_aspect_ratio(mut self, aspect: AspectRatio) -> Self {
        self.aspect_ratio = aspect;
        self
    }

    /// Add a chart title
    pub fn with_title<T>(mut self, title: T) -> Self
    where
        T: Into<Title>,
    {
        self.titles.push(title.into());
        self
    }

    /// Add an `Axis`
    pub fn with_axis<A: Axis + 'a>(mut self, axis: A) -> Self {
        self.axes.push(Box::new(axis));
        self
    }

    /// Add an area `Plot`
    pub fn with_area_plot(mut self, plot: Plot<'a>) -> Self {
        self.plots.push((PlotKind::Area, plot));
        self
    }

    /// Add a line `Plot`
    pub fn with_line_plot(mut self, plot: Plot<'a>) -> Self {
        self.plots.push((PlotKind::Line, plot));
        self
    }

    /// Add a scatter `Plot`
    pub fn with_scatter_plot(mut self, plot: Plot<'a>) -> Self {
        self.plots.push((PlotKind::Scatter, plot));
        self
    }

    fn svg(&self, f: &mut dyn Write, stand_alone: bool) -> fmt::Result {
        let rect = self.aspect_ratio.rect();
        write!(f, "<svg")?;
        if stand_alone {
            write!(f, " xmlns='http://www.w3.org/2000/svg'")?;
        }
        write!(f, " viewBox='")?;
        writeln!(
            f,
            "{} {} {} {}'>",
            rect.x_min(),
            rect.y_min(),
            rect.x_span(),
            rect.y_span()
        )
    }

    fn defs(&self, f: &mut dyn Write) -> fmt::Result {
        writeln!(f, "<defs>")?;
        for i in 0..self.plots.len() {
            write!(f, "<marker id='marker-{}'", i)?;
            write!(f, " class='plot-{}'", i)?;
            write!(f, " viewBox='-1 -1 2 2'")?;
            writeln!(f, " markerWidth='5' markerHeight='5'>")?;
            writeln!(f, "{}", MARKERS[i % MARKERS.len()])?;
            writeln!(f, "</marker>")?;
        }
        let area = self.area();
        writeln!(f, "<clipPath id='clip-chart'>")?;
        write!(f, "<rect x='{}' y='{}'", area.x_min(), area.y_min())?;
        writeln!(f, " width='{}' height='{}'/>", area.x_span(), area.y_span())?;
        writeln!(f, "</clipPath>")?;
        writeln!(f, "</defs>")
    }

    fn body(&mut self, f: &mut dyn Write) -> fmt::Result {
        let mut area = inset(self.aspect_ratio.rect(), 40);
        for title in &self.titles {
            let rect = title.edge.split(&mut area, 100.0);
            title.display(f, rect)?;
        }
        let mut axis_rects = vec![];
        for axis in &self.axes {
            axis_rects.push(axis.split(&mut area));
        }
        for axis in self.axes.iter() {
            axis.display_grid(f, area)?;
        }
        for (axis, rect) in self.axes.iter().zip(axis_rects) {
            axis.display(f, rect, area)?;
        }
        writeln!(f, "<g clip-path='url(#clip-chart)'>")?;
        for ((kind, plot), num) in self.plots.iter_mut().zip((0..10).cycle()) {
            (*plot).display(f, num, area, *kind)?;
        }
        writeln!(f, "</g>")?;
        writeln!(f, "</svg>")
    }

    fn area(&self) -> BBox<f32> {
        let mut area = inset(self.aspect_ratio.rect(), 40);
        for title in &self.titles {
            title.edge.split(&mut area, 100.0);
        }
        for axis in &self.axes {
            axis.split(&mut area);
        }
        area
    }

    /// Render the legend as an HTML fragment
    pub(crate) fn legend(&self, f: &mut dyn Write) -> fmt::Result {
        writeln!(f, "<div class='legend'>")?;
        for (i, plot) in self.plots.iter().enumerate() {
            writeln!(f, "<div>")?;
            writeln!(f, "<svg width='20' height='10' viewBox='0 0 60 30'>")?;
            write!(f, "<path class='plot-{} legend-line'", i)?;
            writeln!(f, " d='M0 15h30h30'/>")?;
            writeln!(f, "</svg>")?;
            writeln!(f, "{}", plot.1.name())?;
            writeln!(f, "</div>")?;
        }
        writeln!(f, "</div>")
    }

    /// Render chart as HTML
    pub fn render(mut self) -> String {
        let mut html = String::new();

        html.push_str("<html>");
        html.push_str("<head>");
        html.push_str("<meta charset='UTF-8'>");
        html.push_str("<link href='./css/splotch.css' rel='stylesheet'/>");
        html.push_str("</head>");
        html.push_str("<body>");
        html.push_str("<div class='page'>");

        // Display chart
        html.push_str("<div class='chart'>");
        self.svg(&mut html, true).unwrap();
        self.defs(&mut html).unwrap();
        self.body(&mut html).unwrap();
        self.legend(&mut html).unwrap();
        html.push_str("</div>");

        html.push_str("</div>");
        html.push_str("</body>");

        html
    }
}

/// Inset bounding box
fn inset(bbox: BBox<f32>, value: u16) -> BBox<f32> {
    // unwrap: Always 2
    let mut iter = bbox.into_iter();
    let mut min = iter.next().unwrap();
    let mut max = iter.next().unwrap();

    min = min + Pt::from(f32::from(value));
    max = max - Pt::from(f32::from(value));

    BBox::from([min, max])
}
