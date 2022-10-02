use splotch::{plot, Chart, Domain, Page};

fn main() {
    let data_a =
        vec![(13.0, 74.0), (111.0, 37.0), (125.0, 52.0), (190.0, 66.0)];
    let data_b =
        vec![(22.0, 50.0), (105.0, 44.0), (120.0, 67.0), (180.0, 39.0)];
    let domain = Domain::from_data(&data_a);
    let plot_a = plot::Scatter::new("Series A", &domain, &data_a);
    let plot_b = plot::Scatter::new("Series B", &domain, &data_b);
    let page = Page::default().with_chart(
        Chart::default()
            .with_title("Scatter Plot")
            .with_axis(domain.x_axis().with_name("X Axis Name"))
            .with_axis(domain.y_axis().with_name("Y Axis Name"))
            .with_axis(domain.y_axis().on_right())
            .with_plot(&plot_a)
            .with_plot(&plot_b),
    );
    print!("{}", page);
}
