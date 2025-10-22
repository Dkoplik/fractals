use std::f32::EPSILON;

use crate::app::logic::transform2d::Transform2D;

/// Линия для рисования
#[derive(Debug, Clone, Default)]
pub struct Line {
    /// Начало линии (отрезка).
    pub begin: egui::Pos2,
    /// Конец линии (отрезка).
    pub end: egui::Pos2,
    /// Ширина линии (отрезка).
    pub width: f32,
    /// Цвет линии (отрезка).
    pub color: egui::Color32,
}

impl Line {
    pub fn draw(&self, painter: &egui::Painter) {
        #[cfg(debug_assertions)]
        println!("Draw line from {:?} to {:?}", self.begin, self.end);
        painter.line(
            vec![self.begin, self.end],
            egui::epaint::PathStroke::new(self.width, self.color),
        );
    }
}

/// Нарисовать коллекцию из линий Line с их масштабированием
pub fn draw_lines(lines: &[Line], painter: &egui::Painter, area: egui::Rect, margin: f32) {
    let sys_rect = find_rect(lines);
    if sys_rect.is_none() {
        #[cfg(debug_assertions)]
        println!("No fractal rect");
        return;
    }
    let sys_rect = sys_rect.unwrap();

    #[cfg(debug_assertions)]
    println!(
        "Bounding box: min: {:?}, max: {:?}, size: {:?}",
        sys_rect.min,
        sys_rect.max,
        sys_rect.size()
    );
    #[cfg(debug_assertions)]
    println!("Screen area: {:?}", area);

    let (scale_tr, move_tr) = get_transform_to_fullscreen(area, sys_rect, margin);

    lines.iter().cloned().for_each(|mut line| {
        line.begin = move_tr.apply_to_pos(scale_tr.apply_to_pos(line.begin));
        line.end = move_tr.apply_to_pos(scale_tr.apply_to_pos(line.end));
        line.draw(painter);
    });
}

/// Найти прямоугольник описывающий узор lines.
pub fn find_rect(lines: &[Line]) -> Option<egui::Rect> {
    let mut pos_min: Option<egui::Pos2> = None;
    let mut pos_max: Option<egui::Pos2> = None;
    for line in lines {
        let x_min = line.begin.x.min(line.end.x);
        let y_min = line.begin.y.min(line.end.y);
        let x_max = line.begin.x.max(line.end.x);
        let y_max = line.begin.y.max(line.end.y);

        if let Some(pos_min) = &mut pos_min {
            pos_min.x = pos_min.x.min(x_min);
            pos_min.y = pos_min.y.min(y_min);
        } else {
            pos_min = Some(egui::Pos2::new(
                line.begin.x.min(line.end.x),
                line.begin.y.min(line.end.y),
            ));
        }

        if let Some(pos_max) = &mut pos_max {
            pos_max.x = pos_max.x.max(x_max);
            pos_max.y = pos_max.y.max(y_max);
        } else {
            pos_max = Some(egui::Pos2::new(
                line.begin.x.max(line.end.x),
                line.begin.y.max(line.end.y),
            ));
        }
    }

    if let Some(pos_min) = pos_min
        && let Some(mut pos_max) = pos_max
    {
        if (pos_min.x - pos_max.x).abs() < EPSILON {
            pos_max.x += 1.0;
        }
        if (pos_min.y - pos_max.y).abs() < EPSILON {
            pos_max.y += 1.0;
        }
        Some(egui::Rect::from_min_max(pos_min, pos_max))
    } else {
        None
    }
}

/// Находит преобразование Transform2D дабы разместить область рисования на всю область экрана.
pub fn get_transform_to_fullscreen(
    screen_rect: egui::Rect,
    draw_rect: egui::Rect,
    margin: f32,
) -> (Transform2D, Transform2D) {
    // scale image
    let scale = ((screen_rect.width() - 2.0 * margin) / draw_rect.width())
        .min((screen_rect.height() - 2.0 * margin) / draw_rect.height());
    #[cfg(debug_assertions)]
    println!("scale: {}", scale);
    let transform = Transform2D::uniform_scaling(scale);

    let mut scaled_rect = draw_rect;
    scaled_rect.min = transform.apply_to_pos(scaled_rect.min);
    scaled_rect.max = transform.apply_to_pos(scaled_rect.max);

    #[cfg(debug_assertions)]
    println!("scaled fractal: {:?}", scaled_rect);
    #[cfg(debug_assertions)]
    println!("screen center: {}", screen_rect.center());
    #[cfg(debug_assertions)]
    println!("fractal center: {}", scaled_rect.center());

    // center image
    let d = screen_rect.center() - scaled_rect.center();
    (transform, Transform2D::translation(d.x, d.y) )
}
