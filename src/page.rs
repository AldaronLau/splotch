// page.rs
//
// Copyright (c) 2021  Douglas P Lau
// Copyright (c) 2022  Jeron A Lau
//
use pointy::BBox;

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

impl AspectRatio {
    pub(crate) fn rect(self) -> BBox<f32> {
        match self {
            AspectRatio::Landscape => BBox::new([(0.0, 0.0), (2000.0, 1500.0)]),
            AspectRatio::Square => BBox::new([(0.0, 0.0), (2000.0, 2000.0)]),
            AspectRatio::Portrait => BBox::new([(0.0, 0.0), (1500.0, 2000.0)]),
        }
    }
}
