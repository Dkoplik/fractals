use crate::app::logic::utils;
use  rand::Rng;

/// Реализация для midpoint displacement.
pub struct MidDisplacement {
    /// Шероховатость (R из формулы на презе).
    roughness: f32,
    /// Текущая итерация.
    iter: u32,
    /// Текущие линии (изображение).
    lines: Vec<utils::Line>,
// base_width: f32,
// base_height: f32
}

impl MidDisplacement {
    pub fn new(roughness: f32) -> Self {
        let starter_line = utils::Line {
            begin: egui::Pos2::new(0.0, 300.0),    // Начало слева по центру
            end: egui::Pos2::new(900.0, 300.0), 
            width: 1.0,
            color: egui::Color32::BLACK,
        };
        Self {
            roughness,
            iter: 0,
            lines: vec![starter_line],
            // base_width: 1.0,  // Не используется в этом подходе
            // base_height: 1.0,
        }
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
        let starter_line = utils::Line {
            begin: egui::Pos2::new(0.0, 300.0),    // Начало слева по центру
            end: egui::Pos2::new(900.0, 300.0), 
            width: 1.0,
            color: egui::Color32::BLACK,
        };
        self.lines = vec![starter_line];
        self.iter = 1;
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
    let random_offset = rng.random_range(0.0 .. 2.0) * random_range - random_range;
    
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
