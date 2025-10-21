use crate::app::logic::utils;
use rand::Rng;

/// Реализация для midpoint displacement.
pub struct MidDisplacement {
    /// Шероховатость (R из формулы на презе).
    roughness: f32,
    /// Текущая итерация.
    iter: u32,
    /// Текущие линии (изображение).
    lines: Vec<utils::Line>,
    /// Начальная линия (для сброса)
    initial_line: utils::Line,
}

impl MidDisplacement {
    pub fn new(roughness: f32) -> Self {
        let initial_line = utils::Line {
            begin: egui::Pos2::new(0.0, 0.5),    // Относительные координаты (0-1)
            end: egui::Pos2::new(1.0, 0.5),
            width: 1.0,
            color: egui::Color32::BLACK,
        };
        Self {
            roughness,
            iter: 0,
            lines: vec![],
            initial_line,
        }
    }

    /// Инициализировать или сбросить линии для заданной области
    pub fn init_for_area(&mut self, area: egui::Rect) {
        // преобразуем относительные координаты в абсолютные для данной области
        let begin = egui::Pos2::new(
            area.left(),
            area.top() + self.initial_line.begin.y * area.height()
        );
        let end = egui::Pos2::new(
            area.left() + area.width(),
            area.top() + self.initial_line.end.y * area.height()
        );
        
        self.lines = vec![utils::Line {
            begin,
            end,
            width: 1.0,
            color: egui::Color32::BLACK,
        }];
        self.iter = 0;
    }

    /// Провести ещё одну итерацию midpoint displacement.
    pub fn iter_once(&mut self) {      
        let mut new_lines = Vec::new();
        
        for line in &self.lines {
            let (left_line, right_line) = split_line(line, self.roughness);
            new_lines.push(left_line);
            new_lines.push(right_line);
        }
        
        self.lines = new_lines;
        self.iter += 1;
    }

    /// Получить номер текущей итерации.
    pub fn cur_iter_num(&self) -> u32 {
        self.iter
    }

    pub fn draw(&self, painter: &egui::Painter, area: egui::Rect) {
        utils::draw_lines(&self.lines, painter, area, 0.0);
    }

    /// Очистить данные
    pub fn clear(&mut self) {
        self.lines.clear();
        self.iter = 1;
    }

    /// Получить текущую ограничивающую рамку всех линий
    pub fn get_bounding_rect(&self) -> Option<egui::Rect> {
        utils::find_rect(&self.lines)
    }
}

/// Разделяет одну линию на 2 для midpoint displacement.
fn split_line(line: &utils::Line, roughness: f32) -> (utils::Line, utils::Line) {
    let begin = line.begin;
    let end = line.end;
    
    let x = (begin.x + end.x) / 2.0;
    let average_height = (begin.y + end.y) / 2.0;
    
    let len = (end.x - begin.x).abs();
    
    let random_range = roughness * len;
    let mut rng = rand::rng();
    let random_offset = rng.random_range(-random_range..random_range);
    
    // h = (hL + hR) / 2 + random(-R * L, R * L)
    let h = average_height + random_offset;
    
    let mid_point = egui::Pos2::new(x, h);
    let left_line = utils::Line {
        begin,
        end: mid_point,
        width: line.width,
        color: line.color,
    };
    let right_line = utils::Line {
        begin: mid_point,
        end,
        width: line.width,
        color: line.color,
    };
    
    (left_line, right_line)
}