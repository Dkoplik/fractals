use eframe::glow::OFFSET;

use crate::app::logic::utils;

/// Составая кубическая кривая Безье.
#[derive(Default)]
pub struct BezierCurve {
    /// Точки составной кривой.
    /// Например, кривая из 2-х частей будет содержать точки P1, P2, P3, P4, P5, P6, P7,
    /// где через P1, P4, P7 проходит кривая и P4 - общая точка для 2-х сегментов.
    points: Vec<egui::Pos2>,
    /// Текущие линии (изображение).
    lines: Vec<utils::Line>,
}

impl BezierCurve {
    /// Шаг визуализации кривой Безье.
    static STEP: f32 = 1.0;
    /// Расположение 2-ой опорной точки относительно 1-ой при их добавлении.
    static OFFSET: egui::Vec2 = egui::Vec2::new(0.0, 15.0);

    fn draw_points(&self, painter: &egui::Painter) {
        for i in (0..self.points.len()).step_by(2) {
            painter.line(
                vec![self.points[i], self.points[i + 1]],
                egui::epaint::PathStroke::new(1.0, egui::Color32::LIGHT_GRAY),
            );
        }
        self.points
            .iter()
            .for_each(|point| painter.circle_filled(point, 3.0, egui::Color32::DARK_BLUE));
    }

    pub fn draw(&self, painter: &egui::Painter) {
        self.lines.iter().for_each(|line| line.draw(painter));
        self.draw_points(painter);
    }

    /// Найти индекс ближайшей опорной точки кривой к указанной позиции в пределах указанного радиуса.
    fn get_point_index(&self, pos: egui::Pos2, r: f32) -> usize {
        // TODO
        0
    }

    /// Найти ближайшую опорную точку кривой к указанной позиции в пределах указанного радиуса.
    pub fn get_point(&self, pos: egui::Pos2, r: f32) -> &egui::Pos2 {
        let index = self.get_point_index(pos, r);
        self.points[index]
    }

    /// Найти ближайшую опорную точку кривой к указанной позиции в пределах указанного радиуса.
    pub fn get_point_mut(&mut self, pos: egui::Pos2, r: f32) -> &mut egui::Pos2 {
        let index = self.get_point_index(pos, r);
        self.points[index]
    }

    /// Обновить кривую.
    pub fn update(&mut self, step: f32) {
        // TODO по опорным точкам построить новый вектор self.lines,
        // где каждая линия примерно длины step (через линии приблизительно рисуем кривую)
        // Замечание, self.points может быть пустым или содержать только 2 точки.
    }

    /// Добавляет в кривую новую точку. Через указанную точку кривая должна проходить,
    /// 2-ая точка в пару добавляется автоматически.
    pub fn add_point(&mut self, pos: egui::Pos2) {
        let pos2 = pos + OFFSET;

        let cur_amount = self.points.len();
        if cur_amount == 0 {
            self.points.push(pos);
            self.points.push(pos2);
        }
        else if cur_amount == 2 {
            self.points.push(pos2);
            self.points.push(pos);
        }
        else {
            let pos3 = *self.points.last().unwrap() + OFFSET;
            self.points.push(pos3);
            self.points.push(pos2);
            self.points.push(pos);
        }
    }

    /// Удалить точку рядом с указанной позицией.
    pub fn delete_point(&mut self, pos: egui::Pos2, r: f32) {
        let index = self.get_point_index(pos, r);

        let beg = 0.max(index - ((index + 1) % 4));
        let end = self.points.len().min(index + (2 - ((index + 1) % 4)));
        self.points.drain(beg..=end);
    }
}
