//! Plot data in footile
//!
//! ## Example Line Plot
//!
//! ```rust
//! use splotch::{plot, Chart, Domain, Page};
//!
//! let data = vec![(13.0, 74.0), (111.0, 37.0), (125.0, 52.0), (190.0, 66.0)];
//! let domain = Domain::from_data(&data).with_x(&[0.0, 200.0]);
//! let plot = plot::Line::new("Series", &domain, &data);
//! let page = Page::default().with_chart(
//!     Chart::default()
//!         .with_title("Line Plot")
//!         .with_axis(domain.x_axis().with_name("X Axis Name"))
//!         .with_axis(domain.y_axis().with_name("Y Axis Name").on_right())
//!         .with_plot(&plot),
//! );
//! println!("{}", page);
//! ```
#![forbid(unsafe_code)]

pub mod axis;
mod chart;
mod domain;
mod page;
pub mod plot;
pub mod scale;
mod text;

pub use chart::{Chart, Title};
pub use domain::Domain;
pub use page::{AspectRatio, Page};
