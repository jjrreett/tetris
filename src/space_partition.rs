use std::collections::HashMap;

use crate::tui::layout::Rect;

#[derive(Debug, Clone)]
pub enum Partition {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
pub enum SizeConstraint {
    Fixed(u16),
    Rest,
}

#[derive(Debug, Clone)]
pub struct Area {
    pub name: String,
    pub partition: Option<Partition>,
    pub size: Option<SizeConstraint>,
    pub areas: Vec<Area>,
}

impl Area {
    pub fn new(name: &str) -> Self {
        Area {
            name: name.to_string(),
            partition: None,
            size: None,
            areas: Vec::new(),
        }
    }

    pub fn with_partition(mut self, partition: Partition) -> Self {
        self.partition = Some(partition);
        self
    }

    pub fn with_size_constraint(mut self, size: SizeConstraint) -> Self {
        self.size = Some(size);
        self
    }

    pub fn add_sub_area(mut self, area: Area) -> Self {
        self.areas.push(area);
        self
    }
}

pub fn calculate_layout(area: &Area, available_space: Rect) -> HashMap<String, Rect> {
    let mut layout = HashMap::new();
    let mut current_offset = match area.partition {
        Some(Partition::Horizontal) => available_space.x,
        Some(Partition::Vertical) => available_space.y,
        None => 0, // No partition means it's a leaf node
    };

    let total_space = match area.partition {
        Some(Partition::Horizontal) => available_space.width,
        Some(Partition::Vertical) => available_space.height,
        None => 0, // No partition means it's a leaf node
    };

    let mut space_remaining = total_space;
    let mut rest_areas_count = 0;

    // First pass to calculate fixed sizes and count rest areas
    for sub_area in &area.areas {
        match sub_area.size {
            Some(SizeConstraint::Fixed(size)) => {
                space_remaining -= size;
            }
            Some(SizeConstraint::Rest) => {
                rest_areas_count += 1;
            }
            _ => {}
        };
    }

    let rest_area_size = if rest_areas_count > 0 {
        space_remaining / rest_areas_count as u16
    } else {
        0
    };

    // Second pass to assign sizes and calculate Rects
    for sub_area in &area.areas {
        let size = match sub_area.size {
            Some(SizeConstraint::Fixed(size)) => size,
            Some(SizeConstraint::Rest) => rest_area_size,
            None => total_space,
        };

        let (width, height) = match area.partition {
            Some(Partition::Horizontal) => (size, available_space.height),
            Some(Partition::Vertical) => (available_space.width, size),
            None => (available_space.width, available_space.height),
        };

        let area_rect = Rect::new(current_offset, available_space.y, width, height);
        layout.insert(sub_area.name.clone(), area_rect);

        current_offset += match area.partition {
            Some(Partition::Horizontal) => width,
            Some(Partition::Vertical) => height,
            None => 0,
        };

        if !sub_area.areas.is_empty() {
            let sub_layout = calculate_layout(sub_area, area_rect);
            layout.extend(sub_layout);
        }
    }

    layout
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_partition_fixed_sizes() {
        let area = Area {
            name: "root".to_string(),
            partition: Some(Partition::Horizontal),
            size: None,
            areas: vec![
                Area {
                    name: "left".to_string(),
                    partition: None,
                    size: Some(SizeConstraint::Fixed(20)),
                    areas: Vec::new(),
                },
                Area {
                    name: "right".to_string(),
                    partition: None,
                    size: Some(SizeConstraint::Rest),
                    areas: Vec::new(),
                },
            ],
        };

        let available_space = Rect::new(0, 0, 50, 10);
        let layouts = calculate_layout(&area, available_space);
        assert_eq!(
            layouts.get("left"),
            Some(&Rect::new(0, 0, 20, available_space.height))
        );
        assert_eq!(
            layouts.get("right"),
            Some(&Rect::new(
                20,
                0,
                available_space.width - 20,
                available_space.height
            ))
        );
        // And so on for other assertions...
    }

    #[test]
    fn test_vertical_partition_with_rest() {
        let area = Area {
            name: "root".to_string(),
            partition: Some(Partition::Vertical),
            size: None,
            areas: vec![
                Area {
                    name: "top".to_string(),
                    partition: None,
                    size: Some(SizeConstraint::Rest),
                    areas: Vec::new(),
                },
                Area {
                    name: "bottom".to_string(),
                    partition: None,
                    size: Some(SizeConstraint::Fixed(20)),
                    areas: Vec::new(),
                },
            ],
        };

        let available_space = Rect::new(0, 0, 10, 50); // Mocked available space for test
        let layouts = calculate_layout(&area, available_space); // Assuming calculate_layout is implemented

        assert_eq!(
            layouts.get("top"),
            Some(&Rect::new(0, 0, available_space.width, 30))
        ); // Height is 50 - 20 for bottom
        assert_eq!(
            layouts.get("bottom"),
            Some(&Rect::new(0, 30, available_space.width, 20))
        ); // Starts at y = 30, height = 20
    }
}
