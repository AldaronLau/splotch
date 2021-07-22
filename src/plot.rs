use crate::domain::Domain;
use crate::page::Rect;
use crate::point::Point;
use crate::private::SealedPlot;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum PlotType {
    Area,
    Line,
    Scatter,
}

pub(crate) trait Plot: SealedPlot {}

pub struct Plotter<'a, P>
where
    P: Point + 'a,
{
    plot_type: PlotType,
    domain: &'a Domain,
    data: &'a [P],
}

impl PlotType {
    fn display<P: Point>(
        &self,
        f: &mut fmt::Formatter,
        plot: &Plotter<P>,
        rect: Rect,
    ) -> fmt::Result {
        let rx = rect.x as f32;
        let ry = rect.y as f32;
        let rw = f32::from(rect.width);
        let rh = f32::from(rect.height);
        for (i, pt) in plot.data.iter().enumerate() {
            let x = rx + rw * plot.domain.x(pt.x());
            let y = ry + rh * plot.domain.y(pt.y());
            if i == 0 {
                write!(f, "M{} {}", x, y)?;
            } else {
                write!(f, " {} {}", x, y)?;
            }
        }
        match self {
            PlotType::Area => {
                if let Some(pt) = plot.data.last() {
                    let x = rx + rw * plot.domain.x(pt.x());
                    let y = ry + rh * plot.domain.y(0.0);
                    write!(f, " {} {}", x, y)?;
                }
                if let Some(pt) = plot.data.first() {
                    let x = rx + rw * plot.domain.x(pt.x());
                    let y = ry + rh * plot.domain.y(0.0);
                    write!(f, " {} {} z", x, y)?;
                }
            }
            _ => (),
        }
        Ok(())
    }
}

impl<'a, P> Plotter<'a, P>
where
    P: Point,
{
    pub fn new(domain: &'a Domain, data: &'a [P]) -> Self {
        Plotter {
            plot_type: PlotType::Line,
            domain,
            data,
        }
    }

    pub fn with_plot_type(mut self, plot_type: PlotType) -> Self {
        self.plot_type = plot_type;
        self
    }
}

impl<'a, P> SealedPlot for Plotter<'a, P>
where
    P: Point,
{
    fn display(
        &self,
        f: &mut fmt::Formatter,
        num: usize,
        rect: Rect,
    ) -> fmt::Result {
        write!(f, "<path class='series-{}' d='", num)?;
        self.plot_type.display(f, self, rect)?;
        writeln!(f, "' />")
    }
}

impl<'a, P> Plot for Plotter<'a, P> where P: Point {}
