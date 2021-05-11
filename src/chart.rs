use crate::axis::Axis;
use crate::text::{Anchor, Text};
use crate::page::{AspectRatio, Edge, Rect};
use std::fmt;

pub struct Title {
    text: String,
    anchor: Anchor,
    edge: Edge,
}

impl<T> From<T> for Title
where
    T: Into<String>,
{
    fn from(text: T) -> Self {
        Title::new(text.into())
    }
}

impl Title {
    pub(crate) fn new_with_edge<T>(text: T, edge: Edge) -> Self
    where
        T: Into<String>,
    {
        Title {
            text: text.into(),
            anchor: Anchor::Middle,
            edge,
        }
    }

    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self::new_with_edge(text, Edge::Top)
    }

    pub fn at_start(mut self) -> Self {
        self.anchor = Anchor::Start;
        self
    }

    pub fn at_end(mut self) -> Self {
        self.anchor = Anchor::End;
        self
    }

    pub fn on_bottom(mut self) -> Self {
        self.edge = Edge::Bottom;
        self
    }

    pub fn on_left(mut self) -> Self {
        self.edge = Edge::Left;
        self
    }

    pub fn on_right(mut self) -> Self {
        self.edge = Edge::Right;
        self
    }

    fn display(&self, f: &mut fmt::Formatter, rect: Rect) -> fmt::Result {
        let text =
            Text::new(&self.text, self.edge, self.anchor).class_name("title");
        text.display(f, rect)
    }
}

pub struct ChartBuilder {
    aspect_ratio: AspectRatio,
    titles: Vec<Title>,
    axes: Vec<Axis>,
}

pub struct Chart {
    aspect_ratio: AspectRatio,
    titles: Vec<Title>,
    axes: Vec<Axis>,
}

impl Default for ChartBuilder {
    fn default() -> Self {
        Self {
            aspect_ratio: AspectRatio::Landscape,
            titles: vec![],
            axes: vec![],
        }
    }
}

impl ChartBuilder {
    pub fn aspect_ratio(mut self, aspect: AspectRatio) -> Self {
        self.aspect_ratio = aspect;
        self
    }

    pub fn title<T>(mut self, title: T) -> Self
    where
        T: Into<Title>,
    {
        self.titles.push(title.into());
        self
    }

    pub fn axis(mut self, axis: Axis) -> Self {
        self.axes.push(axis);
        self
    }

    pub fn build(self) -> Chart {
        Chart {
            aspect_ratio: self.aspect_ratio,
            titles: self.titles,
            axes: self.axes,
        }
    }
}

impl Chart {
    pub fn builder() -> ChartBuilder {
        ChartBuilder::default()
    }

    fn header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rect = self.aspect_ratio.rect();
        writeln!(
            f,
            "<svg xmlns='http://www.w3.org/2000/svg' viewBox='{} {} {} {}'>",
            rect.x, rect.y, rect.width, rect.height,
        )?;
        writeln!(f, "<style>")?;
        writeln!(f, ".title {{ font-size: 25px; }}")?;
        writeln!(f, ".axis {{ font-size: 20px; }}")?;
        writeln!(f, "</style>")?;
        Ok(())
    }

    fn footer(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "</svg>")
    }
}

impl fmt::Display for Chart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.header(f)?;
        let mut area = self.aspect_ratio.rect().inset(10);
        for title in &self.titles {
            let rect = area.split(title.edge, 50);
            title.display(f, rect)?;
        }
        for axis in &self.axes {
            let rect = area.split(axis.edge(), axis.space());
            axis.display(f, rect)?;
        }
        self.footer(f)?;
        Ok(())
    }
}
