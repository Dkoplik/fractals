use crate::app::FractalsApp;
use egui::{Color32, Painter, Pos2, Response, Ui};

pub mod bezier;
pub mod l_system;
pub mod midpoint_displacement;
pub mod transform2d;
pub mod utils;

// --------------------------------------------------
// Обработка области рисования (холст)
// --------------------------------------------------

impl FractalsApp {
    /// Выделить egui::painter на всю свободную область указанного UI элемента.
    pub fn allocate_painter(&mut self, ui: &mut Ui) -> (Response, Painter) {
        let available_size = ui.available_size();
        self.painter_width = available_size.x;
        self.painter_height = available_size.y;

        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(self.painter_width, self.painter_height),
            egui::Sense::click_and_drag(),
        );

        // цвет холста
        painter.rect_filled(response.rect, 0.0, Color32::WHITE);

        (response, painter)
    }

    /// Очистить холст.
    pub fn clear_canvas(&mut self) {
        // Очистка данных для разных типов фракталов
        match self.fractal_type {
            crate::app::FractalType::LSystem => {
                // self.lsystem_data.clear();
            }
            crate::app::FractalType::MidpointDisplacement => {
                self.midpoint_displacement.clear();
            }
            crate::app::FractalType::BezierSpline => {
                self.bezier_curve.clear();
            }
        }
        self.point_count = 0;
        self.current_iteration = 0;
    }

    /// Нарисовать холст с текущим фракталом.
    pub fn draw_canvas(&mut self, painter: &Painter) {
        let area = painter.clip_rect();

        match self.fractal_type {
            crate::app::FractalType::LSystem => {
                painter.text(
                    area.center(),
                    egui::Align2::CENTER_CENTER,
                    "L-система будет здесь",
                    egui::FontId::default(),
                    Color32::BLACK,
                );
            }
            crate::app::FractalType::MidpointDisplacement => {
                self.midpoint_displacement.draw(painter, area);

                if self.md_show_steps {
                    let info_text = format!("Итерация: {}", self.current_iteration,);

                    painter.text(
                        area.left_top() + egui::Vec2::new(10.0, 20.0),
                        egui::Align2::LEFT_TOP,
                        info_text,
                        egui::FontId::proportional(14.0),
                        egui::Color32::DARK_GRAY,
                    );
                }
            }
            crate::app::FractalType::BezierSpline => {
                self.bezier_curve.draw(painter);
            }
        }
    }
}

// --------------------------------------------------
// Обработка управления
// --------------------------------------------------

impl FractalsApp {
    /// Обработать взаимодействие с холстом.
    pub fn handle_input(&mut self, response: &Response) {
        self.handle_click(response);
        self.handle_drag(response);
    }

    /// Обработать клики по холсту.
    fn handle_click(&mut self, response: &Response) {
        if response.clicked_by(egui::PointerButton::Primary) {
            if let Some(pos) = response.hover_pos() {
                match self.fractal_type {
                    crate::app::FractalType::BezierSpline => {
                        self.handle_bezier_click(pos);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Обработать перетаскивание для Безье (MovePoint).
    fn handle_drag(&mut self, response: &Response) {
        // Только если инструмент MovePoint
        if self.fractal_type != crate::app::FractalType::BezierSpline {
            return;
        }

        if self.instrument != crate::app::Instrument::MovePoint {
            return;
        }

        // Если сейчас идёт drag ЛКМ
        if response.dragged_by(egui::PointerButton::Primary) {
            if let Some(pos) = response.hover_pos() {
                if let Some(idx) = self.selected_point {
                    // Перемещаем точку по текущей позиции курсора
                    self.bezier_curve.move_point(idx, pos);
                    self.point_count = self.bezier_curve.points.len();
                }
            }
        } else {
            // Если drag закончен/не идёт — сбросим выделение
            // (при отпускании мыши selected_point станет None)
            if response.hover_pos().is_none() || !response.dragged() {
                // Не всегда нужно стирать тут, можно стирать при mouse up; но безопасно:
                // self.selected_point = None;
            }
        }
    }

    /// Обработать клик для кривых Безье.
    fn handle_bezier_click(&mut self, pos: Pos2) {
        match self.instrument {
            crate::app::Instrument::AddPoint => {
                self.bezier_curve.add_point(pos);
                self.point_count = self.bezier_curve.points.len();
            }
            crate::app::Instrument::RemovePoint => {
                self.bezier_curve.delete_point(pos, 10.0);
                self.point_count = self.bezier_curve.points.len();
            }
            crate::app::Instrument::MovePoint => {
                // Сначала пробуем выбрать ближайшую опорную точку
                self.selected_point = self.bezier_curve.nearest_anchor_index(pos, 10.0);

                // Если опорная не найдена — fallback на любую ближайшую точку
                if self.selected_point.is_none() {
                    self.selected_point = self.bezier_curve.nearest_anchor_index(pos, 10.0);
                }
            }
            _ => {}
        }
    }
}

// --------------------------------------------------
// Методы для работы с фракталами
// --------------------------------------------------

impl FractalsApp {
    /// Загрузить L-систему из файла.
    pub fn load_lsystem(&mut self) {
        // TODO: Реализовать загрузку файла
        println!("Загрузка L-системы...");
    }

    /// Сгенерировать случайное дерево (L-система).
    pub fn generate_random_tree(&mut self) {
        // TODO: Реализовать генерацию случайного дерева
        println!("Генерация случайного дерева...");
    }

    /// Сгенерировать горный массив.
    pub fn generate_mountains(&mut self) {
        self.midpoint_displacement = midpoint_displacement::MidDisplacement::new(self.md_roughness);

        for _ in 0..self.md_iterations {
            self.midpoint_displacement.iter_once();
        }

        self.current_iteration = self.midpoint_displacement.cur_iter_num() as usize;
        println!("Генерация горного массива...");
    }

    /// Выполнить итерацию для текущего фрактала.
    pub fn iterate_fractal(&mut self) {
        match self.fractal_type {
            crate::app::FractalType::LSystem => {
                // if let Some(lsystem) = &mut self.lsystem {
                //     lsystem.iter_once();
                //     self.current_iteration = lsystem.cur_iter_num();
                // }
                println!("Итерация L-системы...");
            }
            crate::app::FractalType::MidpointDisplacement => {
                self.midpoint_displacement.iter_once();
                self.current_iteration = self.midpoint_displacement.cur_iter_num() as usize;
                println!("Итерация Midpoint Displacement...");
            }
            crate::app::FractalType::BezierSpline => {
                // Для Безье итерации не применяются
            }
        }
    }

    /// Сбросить кривые Безье.
    pub fn reset_bezier(&mut self) {
        self.bezier_curve.clear();
        self.point_count = 0;
        self.selected_point = None;
    }
}
