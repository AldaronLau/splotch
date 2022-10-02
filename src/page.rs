// page.rs
//
// Copyright (c) 2021  Douglas P Lau
// Copyright (c) 2022  Jeron A Lau
//
use std::fmt;

use pointy::BBox;

use crate::chart::Chart;

/// Page aspect ratio
#[derive(Clone, Copy)]
pub enum AspectRatio {
    /// Wide rectangular aspect
    Landscape,
    /// Square aspect
    Square,
    /// Tall rectangular aspect
    Portrait,
}

/// Edge of rendered item
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    Top,
    Left,
    Bottom,
    Right,
}

impl Edge {
    pub fn split(self, rect: &mut BBox<f32>, value: f32) -> BBox<f32> {
        match self {
            Edge::Top => {
                let y = rect.y_min();
                let height = rect.y_span() - value;
                let h = rect.y_span() - height;

                *rect = BBox::new([
                    (rect.x_min(), rect.y_min() + h),
                    (rect.x_max(), rect.y_max()),
                ]);

                BBox::new([(rect.x_min(), y), (rect.x_max(), y + h)])
            }
            Edge::Left => {
                let x = rect.x_min();
                let width = rect.x_span() - value;
                let w = rect.x_span() - width;

                *rect = BBox::new([
                    (rect.x_min() + w, rect.y_min()),
                    (rect.x_max(), rect.y_max()),
                ]);

                BBox::new([(x, rect.y_min()), (x + w, rect.y_max())])
            }
            Edge::Bottom => {
                let height = rect.y_span() - value;
                let h = rect.y_span() - height;
                let y = rect.y_min() + height;

                *rect = BBox::new([
                    (rect.x_min(), rect.y_min()),
                    (rect.x_max(), rect.y_min() + height),
                ]);

                BBox::new([(rect.x_min(), y), (rect.x_max(), y + h)])
            }
            Edge::Right => {
                let width = rect.x_span() - value;
                let w = rect.x_span() - width;
                let x = rect.x_min() + width;

                *rect = BBox::new([
                    (rect.x_min(), rect.y_min()),
                    (rect.x_min() + width, rect.y_max()),
                ]);

                BBox::new([(x, rect.y_min()), (x + w, rect.y_max())])
            }
        }
    }
}

/// Page to render charts
///
/// A `Page` containing one or more `Chart`s can be rendered as HTML using the
/// `Display` trait.  That is, using `println!`, or even `to_string()` is all
/// that's needed.
#[derive(Default)]
pub struct Page<'a> {
    charts: Vec<Chart<'a>>,
}

impl AspectRatio {
    pub(crate) fn rect(self) -> BBox<f32> {
        match self {
            AspectRatio::Landscape => BBox::new([(0.0, 0.0), (2000.0, 1500.0)]),
            AspectRatio::Square => BBox::new([(0.0, 0.0), (2000.0, 2000.0)]),
            AspectRatio::Portrait => BBox::new([(0.0, 0.0), (1500.0, 2000.0)]),
        }
    }
}

impl<'a> Page<'a> {
    /// Add a `Chart` to `Page`
    pub fn with_chart(mut self, chart: Chart<'a>) -> Self {
        self.charts.push(chart);
        self
    }
}

impl<'a> fmt::Display for Page<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<html>")?;
        writeln!(f, "<head>")?;
        writeln!(f, "<meta charset='UTF-8'>")?;
        writeln!(f, "<link href='./css/splotch.css' rel='stylesheet'/>")?;
        writeln!(f, "</head>")?;
        writeln!(f, "<body>")?;
        writeln!(f, "<div class='page'>")?;
        for chart in &self.charts {
            writeln!(f, "<div class='chart'>")?;
            chart.display(f)?;
            chart.legend(f)?;
            writeln!(f, "</div>")?;
        }
        writeln!(f, "</div>")?;
        writeln!(f, "</body>")?;
        Ok(())
    }
}
