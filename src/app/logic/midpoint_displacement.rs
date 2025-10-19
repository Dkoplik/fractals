use crate::app::logic::utils;

/// Реализация для midpoint displacement.
pub struct MidDisplacement {
    /// Шероховатость (R из формулы на презе).
    roughness: f32,
    /// Цвет линии.
    color: egui::Color32,
    /// Текущая итерация.
    iter: u32,
    /// Текущие линии (изображение).
    lines: Vec<utils::Line>,
}

impl MidDisplacement {
    pub fn new(roughness: f32, color: egui::Color32) -> Self {
        let starter_line = utils::Line {
            begin: egui::Pos2::new(0.0, 0.0),
            end: egui::Pos2::new(1.0, 0.0),
            width: 1.0,
            color,
        };
        Self {
            roughness,
            color,
            iter: 1,
            lines: vec![starter_line],
        }
    }

    /// Провести ещё одну итерацию midpoint displacement.
    pub fn iter_once(&mut self) {
        // TODO использовать функцию split_line в реализации
        self.iter += 1; // не забыть счётчик итераций увеличить
    }

    /// Получить номер текущей итерации.
    pub fn cur_iter_num(&self) -> u32 {
        self.iter
    }

    pub fn draw(&self, painter: &egui::Painter, area: egui::Rect, margin: f32) {
        utils::draw_lines(&self.lines, painter, area, margin);
    }
}

/// Разделяет одну линию на 2 для midpoint displacement.
fn split_line(line: &utils::Line, roughness: f32) -> (utils::Line, utils::Line) {
    // TODO по сути 1 шаг (операция над одной линией из всех) в midpoint displacement, интерфейс самому подогнать под нужный.
    (utils::Line::default(), utils::Line::default())
}
