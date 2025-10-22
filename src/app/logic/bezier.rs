use crate::app::logic::utils;

#[derive(Default)]
pub struct BezierCurve {
    /// Все точки кривой: опорные и контрольные.
    pub points: Vec<egui::Pos2>,
    /// Линии для визуализации.
    lines: Vec<utils::Line>,
}

impl BezierCurve {
    const STEP: f32 = 0.02;

    fn draw_points(&self, painter: &egui::Painter) {
        for (i, point) in self.points.iter().enumerate() {
            let color = if i % 3 == 0 {
                egui::Color32::LIGHT_GREEN
            } else {
                egui::Color32::LIGHT_RED
            };
            painter.circle_filled(*point, 4.0, color);

            //if i % 3 != 0 {
            //    painter.circle_filled(*point, 4.0, egui::Color32::LIGHT_RED);
            //}
        }
    }

    pub fn draw(&self, painter: &egui::Painter) {
        // Рисование контрольных линий
        for i in (0..self.points.len()).step_by(3) {
            if i + 1 < self.points.len() {
                painter.line_segment(
                    [self.points[i], self.points[i + 1]],
                    egui::Stroke::new(1.0, egui::Color32::LIGHT_GRAY),
                );
            }

            if i + 1 < self.points.len() && i + 2 < self.points.len() {
                painter.line_segment(
                    [self.points[i + 1], self.points[i + 2]],
                    egui::Stroke::new(1.0, egui::Color32::LIGHT_GRAY),
                );
            }

            if i + 2 < self.points.len() && i + 3 < self.points.len() {
                painter.line_segment(
                    [self.points[i + 2], self.points[i + 3]],
                    egui::Stroke::new(1.0, egui::Color32::LIGHT_GRAY),
                );
            }
        }

        // Рисование кривых Безье
        for line in &self.lines {
            line.draw(painter);
        }

        self.draw_points(painter);
    }

    pub fn nearest_anchor_index(&self, pos: egui::Pos2, r: f32) -> Option<usize> {
        if self.points.is_empty() {
            return None;
        }
        let mut best: Option<usize> = None;
        let mut best_d2 = r * r;
        for i in (0..self.points.len()).step_by(3) {
            let d2 = self.points[i].distance_sq(pos);
            if d2 < best_d2 {
                best_d2 = d2;
                best = Some(i);
            }
        }
        best
    }

    fn bezier_point(
        p0: egui::Pos2,
        p1: egui::Pos2,
        p2: egui::Pos2,
        p3: egui::Pos2,
        t: f32,
    ) -> egui::Pos2 {
        let u = 1.0 - t;
        let u2 = u * u;
        let u3 = u2 * u;
        let t2 = t * t;
        let t3 = t2 * t;
        egui::pos2(
            u3 * p0.x + 3.0 * u2 * t * p1.x + 3.0 * u * t2 * p2.x + t3 * p3.x,
            u3 * p0.y + 3.0 * u2 * t * p1.y + 3.0 * u * t2 * p2.y + t3 * p3.y,
        )
    }

    pub fn update(&mut self) {
        self.lines.clear();
        if self.points.len() < 4 {
            return;
        }

        for i in (0..self.points.len() - 3).step_by(3) {
            let p0 = self.points[i];
            let p1 = self.points[i + 1];
            let p2 = self.points[i + 2];
            let p3 = self.points[i + 3];

            let mut prev = p0;
            let mut t = Self::STEP;
            while t <= 1.0 {
                let cur = Self::bezier_point(p0, p1, p2, p3, t);
                self.lines.push(utils::Line {
                    begin: prev,
                    end: cur,
                    width: 2.0,
                    color: egui::Color32::BLACK,
                });
                prev = cur;
                t += Self::STEP;
            }
        }
    }

    // вычисляет контрольные точки для обеспечения C¹-непрерывности
    fn calculate_control_points(&mut self) {
        if self.points.len() < 4 {
            return;
        }

        for i in (3..self.points.len() - 3).step_by(3) {
            let prev_anchor = self.points[i - 3];
            let curr_anchor = self.points[i];
            let next_anchor = self.points[i + 3];

            let tangent_direction = (next_anchor - prev_anchor) * 0.25;

            self.points[i - 1] = curr_anchor - tangent_direction;
            self.points[i + 1] = curr_anchor + tangent_direction;
        }

        if self.points.len() >= 4 {
            let first_anchor = self.points[0];
            let second_anchor = self.points[3];
            let first_tangent = (second_anchor - first_anchor) * 0.3;
            self.points[1] = first_anchor + first_tangent;

            let last_index = self.points.len() - 1;
            let last_anchor = self.points[last_index];
            let prev_anchor = self.points[last_index - 3];
            let last_tangent = (last_anchor - prev_anchor) * 0.3;
            self.points[last_index - 1] = last_anchor - last_tangent;
        }
    }

    pub fn add_point(&mut self, click_pos: egui::Pos2) {
        let n = self.points.len();

        match n {
            0 => {
                // Первая точка - просто добавляем опорную
                self.points.push(click_pos);
            }
            1 => {
                // Вторая точка - добавляем первую контрольную и вторую опорную
                let first_anchor = self.points[0];
                let control_offset = (click_pos - first_anchor) * 0.3;

                self.points.push(first_anchor + control_offset);
                self.points.push(click_pos - control_offset);
                self.points.push(click_pos); // Опорная
            }
            _ => {
                // Добавляем новую опорную точку
                let last_anchor_index = ((self.points.len() - 1) / 3) * 3;
                let last_anchor = self.points[last_anchor_index];

                // Вычисляем контрольные точки
                let control_offset = (click_pos - last_anchor) * 0.3;

                self.points.push(last_anchor + control_offset);
                self.points.push(click_pos - control_offset);
                self.points.push(click_pos); // Опорная

                // Пересчитываем все контрольные точки для гладкости
                self.calculate_control_points();
            }
        }

        self.update();
    }

    pub fn delete_point(&mut self, pos: egui::Pos2, r: f32) {
        if self.points.len() < 4 {
            self.clear();
            return;
        }
        if let Some(idx) = self.nearest_anchor_index(pos, r) {
            if idx % 3 != 0 {
                return;
            }
            if idx == 0 {
                let end = (idx + 3).min(self.points.len());
                self.points.drain(idx..end);
            } else if idx + 1 == self.points.len() {
                self.points.drain((idx - 2)..idx + 1);
            } else {
                self.points.drain((idx - 1)..(idx + 2));
            }
            self.update();
        }
    }

    pub fn move_point(&mut self, index: usize, new_pos: egui::Pos2) {
        if index >= self.points.len() {
            return;
        }

        let delta = new_pos - self.points[index];
        self.points[index] = new_pos;

        // Если index — опорная точка, сдвинем соседние control
        if index % 3 == 0 {
            if index > 0 {
                self.points[index - 1] += delta;
            }
            if index + 1 < self.points.len() {
                self.points[index + 1] += delta;
            }
        }

        self.update();
    }

    pub fn clear(&mut self) {
        self.points.clear();
        self.lines.clear();
    }
}
