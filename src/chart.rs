//! # Chart
//!
//! A chart consists of a [View] and between 2 and 4 [Axis]
//! that define the bounds of the [Dataset]s present in the [View].
//!
//! Charts are the smallest self-sufficient entities that can be saved as a file.

use std::ffi::OsStr;
use std::path::Path;
use svg;
use svg::parser::Error;
use svg::node::element::Group;
use svg::Node;
use crate::view::View;
use crate::{XAxis, YAxis};

/// The BarChart struct definition.
pub struct Chart<'a> {
    margin_top: usize,
    margin_bottom: usize,
    margin_right: usize,
    margin_left: usize,
    width: usize,
    height: usize,
    x_axis_bottom: Option<&'a dyn XAxis>,
    y_axis_left: Option<&'a dyn YAxis>,
    view: &'a View<'a>,
}

impl<'a> Chart<'a> {
    pub fn with_view(view: &'a View<'a>) -> Self {
        Self {
            margin_top: 20,
            margin_bottom: 50,
            margin_right: 20,
            margin_left: 60,
            width: 800,
            height: 600,
            x_axis_bottom: None,
            y_axis_left: None,
            view,
        }
    }

    pub fn add_bottom_axis(&mut self, axis: &'a dyn XAxis) {
        self.x_axis_bottom = Some(axis);
    }

    pub fn add_left_axis(&mut self, axis: &'a dyn YAxis) {
        self.y_axis_left = Some(axis);
    }

    fn to_svg(&self) -> Result<Group, Error> {
        let mut group = Group::new()
            .set("class", "g-chart");

        let mut view_group = self.view.to_svg()?;
        view_group.assign("transform", format!("translate({},{})", self.margin_left, self.margin_top));
        group.append(view_group);

        if let Some(mut axis) = self.x_axis_bottom {
            let mut axis_group = axis.to_svg().unwrap();
            axis_group.assign("transform", format!("translate({},{})", self.margin_left, self.height - self.margin_bottom));
            group.append(axis_group);
        };

        if let Some(mut axis) = self.y_axis_left {
            let mut axis_group = axis.to_svg().unwrap();
            axis_group.assign("transform", format!("translate({},{})", self.margin_left, self.margin_top));
            group.append(axis_group);
        };

        Ok(group)
    }

    /// Save the chart to a file
    pub fn save(&self, path: &dyn AsRef<Path>) -> Result<(), String> {
        match path.as_ref().extension().and_then(OsStr::to_str) {
            Some("svg") => {
                match self.to_svg() {
                    Ok(svg_content) => {
                        let document = svg::Document::new()
                            .set("width", self.width)
                            .set("height", self.height)
                            .set("viewBox", (0, 0, self.width, self.height))
                            .add(svg_content);

                        svg::save(path, &document).unwrap()
                    },
                    Err(e) => return Err(format!("Encountered an error while saving the chart: {:?}", e)),
                }
            },
            _ => {},
        };
        Ok(())
    }
}