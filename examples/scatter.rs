use pointy::BBox;
use splotch::{
    axis::{Horizontal, Vertical},
    Chart, Plot,
};

fn main() {
    let data_a =
        vec![(13.0, 74.0), (111.0, 37.0), (125.0, 52.0), (190.0, 66.0)];
    let data_b =
        vec![(22.0, 50.0), (105.0, 44.0), (120.0, 67.0), (180.0, 39.0)];
    let domain = BBox::new(data_a.iter().cloned());
    let mut data_a = data_a.into_iter().map(Into::into);
    let mut data_b = data_b.into_iter().map(Into::into);
    let plot_a = Plot::new("Series A", &domain, &mut data_a);
    let plot_b = Plot::new("Series B", &domain, &mut data_b);
    let chart = Chart::default()
        .with_title("Scatter Plot")
        .with_axis(Horizontal::new(domain).with_name("X Axis Name"))
        .with_axis(Vertical::new(domain).with_name("Y Axis Name"))
        .with_axis(Vertical::new(domain).on_right())
        .with_scatter_plot(plot_a)
        .with_scatter_plot(plot_b)
        .render();
    print!("{chart}");
}
