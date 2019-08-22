use crate::browser::tab::point::Point;
use crate::protocol::page;
use crate::protocol::types::JsUInt;

#[derive(Debug, Copy, Clone)]
pub struct ElementQuad {
    pub top_left: Point,
    pub top_right: Point,
    pub bottom_left: Point,
    pub bottom_right: Point,
}

impl ElementQuad {
    pub fn from_raw_points(raw_quad: &[f64; 8]) -> Self {
        Self {
            top_left: Point {
                x: raw_quad[0],
                y: raw_quad[1],
            },
            top_right: Point {
                x: raw_quad[2],
                y: raw_quad[3],
            },
            bottom_right: Point {
                x: raw_quad[4],
                y: raw_quad[5],
            },
            bottom_left: Point {
                x: raw_quad[6],
                y: raw_quad[7],
            },
        }
    }

    pub fn height(&self) -> f64 {
        self.bottom_left.y - self.top_left.y
    }

    pub fn width(&self) -> f64 {
        self.top_right.x - self.top_left.x
    }

    /// The width divided by the height
    pub fn aspect_ratio(&self) -> f64 {
        self.width() / self.height()
    }

    /// The most left (smallest) x-coordinate
    pub fn most_left(&self) -> f64 {
        self.top_right
            .x
            .min(self.top_left.x)
            .min(self.bottom_right.x)
            .min(self.bottom_left.x)
    }

    /// The most right (largest) x-coordinate
    pub fn most_right(&self) -> f64 {
        self.top_right
            .x
            .max(self.top_left.x)
            .max(self.bottom_right.x)
            .max(self.bottom_left.x)
    }

    /// The most top (smallest) y-coordinate
    pub fn most_top(&self) -> f64 {
        self.top_right
            .y
            .min(self.top_left.y)
            .min(self.bottom_right.y)
            .min(self.bottom_left.y)
    }

    /// The most bottom (largest) y-coordinate
    fn most_bottom(&self) -> f64 {
        self.top_right
            .y
            .max(self.top_left.y)
            .max(self.bottom_right.y)
            .max(self.bottom_left.y)
    }

    /// If the most bottom point of `self` is above the most top point of `other`
    pub fn strictly_above(&self, other: &Self) -> bool {
        self.most_bottom() < other.most_top()
    }

    /// If the most bottom point of `self` is above or on the same line as the
    /// most top point of `other`
    pub fn above(&self, other: &Self) -> bool {
        self.most_bottom() <= other.most_top()
    }

    /// If the most top point of `self` is below the most bottom point of `other`
    pub fn strictly_below(&self, other: &Self) -> bool {
        self.most_top() > other.most_bottom()
    }

    /// If the most top point of `self` is below or on the same line as the
    /// most bottom point of `other`
    pub fn below(&self, other: &Self) -> bool {
        self.most_top() >= other.most_bottom()
    }

    /// If the most right point of `self` is left of the most left point of `other`
    pub fn strictly_left_of(&self, other: &Self) -> bool {
        self.most_right() < other.most_left()
    }

    /// If the most right point of `self` is left or on the same line as the
    /// most left point of `other`
    pub fn left_of(&self, other: &Self) -> bool {
        self.most_right() <= other.most_left()
    }

    /// If the most left point of `self` is right of the most right point of `other`
    pub fn strictly_right_of(&self, other: &Self) -> bool {
        self.most_left() > other.most_right()
    }

    /// If the most left point of `self` is right or on the same line as the
    /// most right point of `other`
    pub fn right_of(&self, other: &Self) -> bool {
        self.most_left() >= other.most_right()
    }

    /// If `self` is within the left/right boundaries defined by `other`.
    pub fn within_horizontal_bounds_of(&self, other: &Self) -> bool {
        self.most_left() >= other.most_left() && self.most_right() <= other.most_right()
    }

    /// If `self` is within the top/bottom boundaries defined by `other`.
    pub fn within_vertical_bounds_of(&self, other: &Self) -> bool {
        self.most_top() >= other.most_top() && self.most_bottom() <= other.most_bottom()
    }

    /// If `self` is within the boundaries defined by `other`.
    pub fn within_bounds_of(&self, other: &Self) -> bool {
        self.within_horizontal_bounds_of(&other) && self.within_vertical_bounds_of(&other)
    }
}

#[derive(Debug, Clone)]
pub struct BoxModel {
    pub content: ElementQuad,
    pub padding: ElementQuad,
    pub border: ElementQuad,
    pub margin: ElementQuad,
    pub width: JsUInt,
    pub height: JsUInt,
}

impl BoxModel {
    /// Create a `page::Viewport` equal to the content-box, using a scale of 1.0
    pub fn content_viewport(&self) -> page::Viewport {
        page::Viewport {
            x: self.content.top_left.x,
            y: self.content.top_left.y,
            width: self.content.width(),
            height: self.content.height(),
            scale: 1.0,
        }
    }

    /// Create a `page::Viewport` equal to the padding-box, using a scale of 1.0
    pub fn padding_viewport(&self) -> page::Viewport {
        page::Viewport {
            x: self.padding.top_left.x,
            y: self.padding.top_left.y,
            width: self.padding.width(),
            height: self.padding.height(),
            scale: 1.0,
        }
    }

    /// Create a `page::Viewport` equal to the border-box, using a scale of 1.0
    pub fn border_viewport(&self) -> page::Viewport {
        page::Viewport {
            x: self.border.top_left.x,
            y: self.border.top_left.y,
            width: self.border.width(),
            height: self.border.height(),
            scale: 1.0,
        }
    }

    /// Create a `page::Viewport` equal to the margin-box, using a scale of 1.0
    pub fn margin_viewport(&self) -> page::Viewport {
        page::Viewport {
            x: self.margin.top_left.x,
            y: self.margin.top_left.y,
            width: self.margin.width(),
            height: self.margin.height(),
            scale: 1.0,
        }
    }
}
