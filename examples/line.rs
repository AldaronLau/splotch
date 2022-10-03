use pointy::BBox;
use splotch::{
    axis::{Horizontal, Vertical},
    plot, Chart
};

fn main() {
    let data_a =
        vec![(13.0, 74.0), (111.0, 37.0), (125.0, 52.0), (190.0, 66.0)];
    let data_b = vec![
        (22.0, 50.0),
        (105.0, 44.0),
        (120.0, 67.0),
        (180.0, 39.0),
        (210.0, 43.0),
    ];
    let domain = {
        let mut domain = BBox::new(data_a.iter().cloned());
        domain.extend(data_b.iter().cloned());
        domain
    };
    let plot_a = plot::Line::new("Series A", &domain, &data_a);
    let plot_b = plot::Line::new("Series B", &domain, &data_b);
    let chart = Chart::default()
        .with_title("Line Plot")
        .with_axis(Horizontal::new(domain).with_name("X Axis Name"))
        .with_axis(Vertical::new(domain).with_name("Y Axis Name"))
        .with_axis(Vertical::new(domain).on_right())
        .with_plot(&plot_a)
        .with_plot(&plot_b);
    print!("{chart}");
}
