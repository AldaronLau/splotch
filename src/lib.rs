//! Plot data in footile
//!
//! ## Example Line Plot
//!
//! ```rust
//! use splotch::{Chart, Plot, axis::{Horizontal, Vertical}};
//! use pointy::BBox;
//!
//! let data = vec![(13.0, 74.0), (111.0, 37.0), (125.0, 52.0), (190.0, 66.0)];
//! let domain = {
//!     let mut domain = BBox::new(data.iter().cloned());
//!     domain.extend([(0.0, 200.0)]);
//!     domain
//! };
//! let mut data = data.into_iter().map(Into::into);
//! let plot = Plot::new("Series", &domain, &mut data);
//! let chart = Chart::default()
//!     .with_title("Line Plot")
//!     .with_axis(Horizontal::new(domain).with_name("X Axis Name"))
//!     .with_axis(Vertical::new(domain).with_name("Y Axis Name").on_right())
//!     .with_line_plot(plot)
//!     .render();
//!
//! println!("{chart}");
//! ```
#![forbid(unsafe_code)]

pub mod axis;
mod chart;
mod page;
mod plot;
mod scale;
mod text;

pub use chart::{Chart, Title};
pub use page::AspectRatio;
pub use plot::Plot;
