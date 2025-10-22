use rand::Rng;

use crate::app::logic::utils;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Интерпретации символов.
enum Actions {
    /// Рисование вперёд на указанное расстояние.
    Forward(f32),
    /// Рисование вперёд на случайное расстояние в указанном диапазоне.
    ForwardRandom(f32, f32),
    /// Поворот на указанный угол (в градусах).
    Rotate(f32),
    /// Поворот на случайный угол (в градусах) в указанном диапазоне.
    RotateRandom(f32, f32),
    /// Сохранить текущее состояние.
    Save,
    /// Восстановить предыдущее состояние.
    Restore,
}

impl ToString for Actions {
    fn to_string(&self) -> String {
        match self {
            Self::Forward(x) => format!("FORWARD {}", x),
            Self::ForwardRandom(x, y) => format!("FORWARD RANDOM {} {}", x, y),
            Self::Rotate(x) => format!("ROTATE {}", x),
            Self::RotateRandom(x, y) => format!("ROTATE RANDOM {} {}", x, y),
            Self::Save => format!("SAVE"),
            Self::Restore => format!("RESTORE"),
        }
    }
}

/// Конфигурация L-системы.
pub struct LSystemConfig {
    /// Аксиома (начальное состояние) L-системы.
    axiom: String,
    /// Правила преобразования символов.
    rules: HashMap<char, String>,
    /// Действия, привязанные к символам.
    actions: HashMap<char, Actions>,
    /// Начальная толщина рисуемых линий.
    width: f32,
    /// Изменение толщины после каждой нарисованной линии.
    width_delta: f32,
    /// Изначальный угол поворота (в градусах).
    angle: f32,
    /// Начальный цвет линий.
    color: egui::Color32,
    /// Изменение цвета после каждой нарисованной линии.
    color_delta: egui::Color32,
}

impl ToString for LSystemConfig {
    fn to_string(&self) -> String {
        let mut string = String::new();

        string += "Аксиома: ";
        string += &self.axiom;
        string += "\n";

        string += "Начальный угол: ";
        string += &self.angle.to_string();
        string += "\n";

        string += "Ширина: ";
        string += &self.width.to_string();
        string += "\n";

        string += "Изменение ширины: ";
        string += &self.width_delta.to_string();
        string += "\n";

        string += "Цвет: ";
        string += &format!("{:?}", self.color);
        string += "\n";

        string += "Изменение цвета: ";
        string += &format!("{:?}", self.color_delta);
        string += "\n";

        string += "Правила:\n";
        for rule in &self.rules {
            string.push(*rule.0);
            string += "->";
            string += rule.1;
            string.push('\n');
        }
        string.push('\n');

        string += "Действия:\n";
        for rule in &self.actions {
            string.push(*rule.0);
            string += "->";
            string += &rule.1.to_string();
            string.push('\n');
        }
        string.push('\n');

        string
    }
}

/// Текущая L-система с конфигурацией и итерацией.
pub struct Lsystem {
    /// Конфигурация.
    config: LSystemConfig,
    /// Текущая строка.
    cur_string: String,
    /// Текущая итерация.
    iter: usize,
    /// Текущие линии (изображение).
    lines: Vec<utils::Line>,
}

impl Lsystem {
    pub fn new(config: LSystemConfig) -> Self {
        let mut l = Self {
            cur_string: config.axiom.clone(),
            iter: 1,
            config,
            lines: Vec::new(),
        };
        l.update_lines();
        l
    }
    fn update_lines(&mut self) {
        let mut rng = rand::rng();
        self.lines.clear();
        /// Текущее состояние L-системы
        struct LState {
            /// Текущая позиция.
            pos: egui::Pos2,
            /// Текущий угол поворота (в градусах).
            angle: f32,
            /// Текущая толщина рисуемых линий.
            width: f32,
            /// Текущий цвет рисуемых линий.
            color: egui::Color32,
        }

        let mut state_stack: Vec<LState> = Vec::new();
        let mut current_state = LState {
            pos: egui::Pos2::new(0.0, 0.0),
            angle: self.config.angle,
            width: self.config.width,
            color: self.config.color,
        };

        for ch in self.cur_string.chars() {
            if let Some(action) = self.config.actions.get(&ch) {
                match action {
                    Actions::Forward(distance) => {
                        let new_pos = self.calculate_new_position(
                            current_state.pos,
                            current_state.angle,
                            *distance,
                        );
                        self.lines.push(utils::Line {
                            begin: current_state.pos,
                            end: new_pos,
                            width: current_state.width,
                            color: current_state.color,
                        });
                        current_state.pos = new_pos;
                        current_state.width =
                            (current_state.width + self.config.width_delta).max(1.0);
                        current_state.color = self.add_color_delta(current_state.color);
                    }
                    Actions::ForwardRandom(min, max) => {
                        let distance = rng.random_range(*min..*max);
                        let new_pos = self.calculate_new_position(
                            current_state.pos,
                            current_state.angle,
                            distance,
                        );
                        self.lines.push(utils::Line {
                            begin: current_state.pos,
                            end: new_pos,
                            width: current_state.width,
                            color: current_state.color,
                        });
                        current_state.pos = new_pos;
                        current_state.width =
                            (current_state.width + self.config.width_delta).max(1.0);
                        current_state.color = self.add_color_delta(current_state.color);
                    }
                    Actions::Rotate(angle) => {
                        current_state.angle += angle;
                    }
                    Actions::RotateRandom(min, max) => {
                        let angle = rng.random_range(*min..*max);
                        current_state.angle += angle;
                    }
                    Actions::Save => {
                        state_stack.push(LState {
                            pos: current_state.pos,
                            angle: current_state.angle,
                            width: current_state.width,
                            color: current_state.color,
                        });
                    }
                    Actions::Restore => {
                        if let Some(prev_state) = state_stack.pop() {
                            current_state = prev_state;
                        }
                    }
                }
            }
        }
    }

    /// Провести ещё одну итерацию L-системы.
    pub fn iter_once(&mut self) {
        let mut new_string = String::new();

        for ch in self.cur_string.chars() {
            if let Some(replacement) = self.config.rules.get(&ch) {
                new_string.push_str(replacement);
            } else {
                new_string.push(ch);
            }
        }

        self.cur_string = new_string;
        self.update_lines();
        self.iter += 1;
    }

    /// Следующая позиция чертёжника
    fn calculate_new_position(
        &self,
        start: egui::Pos2,
        angle_degrees: f32,
        distance: f32,
    ) -> egui::Pos2 {
        let angle_radians = angle_degrees.to_radians();
        let dx = distance * angle_radians.cos();
        let dy = distance * angle_radians.sin();
        egui::Pos2::new(start.x + dx, start.y + dy)
    }

    fn add_color_delta(&self, color: egui::Color32) -> egui::Color32 {
        #[cfg(debug_assertions)]
        println!("color before delta: {:?}", color);

        let delta = self.config.color_delta;
        let r = if delta.r() > 0 && color.r() < 255 {
            (color.r() as i32 + delta.r() as i32).clamp(0, 255) as u8
        } else {
            color.r()
        };
        let g = if delta.g() > 0 && color.g() < 255 {
            (color.g() as i32 + delta.g() as i32).clamp(0, 255) as u8
        } else {
            color.g()
        };
        let b = if delta.b() > 0 && color.b() < 255 {
            (color.b() as i32 + delta.b() as i32).clamp(0, 255) as u8
        } else {
            color.b()
        };
        let new_color = egui::Color32::from_rgb(r, g, b);

        #[cfg(debug_assertions)]
        println!("color after delta: {:?}", color);

        new_color
    }

    /// Получить номер текущей итерации.
    pub fn cur_iter_num(&self) -> usize {
        self.iter
    }

    pub fn draw(&self, painter: &egui::Painter, area: egui::Rect, margin: f32) {
        utils::draw_lines(&self.lines, painter, area, margin);
    }
}

impl ToString for Lsystem {
    fn to_string(&self) -> String {
        format!("Итерация: {}\n", self.iter) + &self.config.to_string()
    }
}

/*

Вид файла конфигурации для L-системы:
```
<аксиома> <угол поворота в градусах> <начальное направление в градусах>
<правило/действие/присвоение 1>
...
<правило/действие/присвоение N>
```

Правила имеют вид `<символ> -> <строка над алфавитом>`
Действия имеют вид `<символ> -> <операция>`
Присвоения имеют вид `<переменная> = <значение>` - все они опциональны

Операции имеют следующий вид:
FORWARD <длина>
| FORWARD RANDOM <мин. длина> <макс. длина>
| ROTATE
| ROTATE RANDOM <дельта-угол>
| SAVE
| RESTORE

Переменные могут быть следующие:
WIDTH
| WIDTH_DELTA
| COLOR
| COLOR_DELTA
*/

/// Ошибки во время пасринга конфига L-системы.
#[derive(Debug)]
pub enum LParseErr {
    /// В пределах строки количество элементов отличается от ожидаемого.
    UnexpectedValuesAmount(usize, usize, usize),
    /// Полученное значение не соответсвует ожидаемому.
    UnexpectedValue(usize, String, String),
}

impl std::fmt::Display for LParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnexpectedValuesAmount(line, expected, got) => write!(
                f,
                "строка {}: ожидалось {} элементов, получено {}",
                line, expected, got
            ),
            Self::UnexpectedValue(line, expected, got) => write!(
                f,
                "строка {}: ожидалось {}, получено {}",
                line, expected, got
            ),
        }
    }
}

impl std::error::Error for LParseErr {}

pub struct Parser {
    cur_line: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self { cur_line: 0 }
    }

    /// Простой парсинг файла с конфигурацией L-системы
    pub fn parse_l_system(
        &mut self,
        file_path: impl AsRef<Path>,
    ) -> Result<LSystemConfig, Box<dyn std::error::Error>> {
        let op_keywords = vec!["FORWARD", "ROTATE", "SAVE", "RESTORE"];
        let var_keywords = vec!["WIDTH", "WIDTH_DELTA", "COLOR", "COLOR_DELTA"];

        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);

        // <аксиома> <угол поворота в градусах> <начальное направление в градусах>
        let mut line = String::new();
        reader.read_line(&mut line)?;
        self.cur_line += 1;
        let first_line_elems: Vec<&str> = line.split_whitespace().collect();
        if first_line_elems.len() != 3 {
            return Err(Box::new(LParseErr::UnexpectedValuesAmount(
                self.cur_line,
                3,
                first_line_elems.len(),
            )));
        }
        let axiom = first_line_elems[0].into();
        let rotate_angle: f32 = first_line_elems[1].parse()?;
        let angle = first_line_elems[2].parse()?;

        // <правило> | <действие> | <присвоение>
        let mut rules = HashMap::new();
        let mut actions = HashMap::new();
        let mut width = 1.0;
        let mut width_delta = 0.0;
        let mut color = egui::Color32::BLACK;
        let mut color_delta = egui::Color32::from_rgb(0, 0, 0);
        line.clear();
        while reader.read_line(&mut line)? > 0 {
            self.cur_line += 1;

            // <символ> -> <строка над алфавитом> | <символ> -> <операция>
            if line.contains("->") {
                let elems: Vec<&str> = line.split("->").collect();
                if elems.len() != 2 {
                    return Err(Box::new(LParseErr::UnexpectedValuesAmount(
                        self.cur_line,
                        3,
                        elems.len(),
                    )));
                }

                // <символ>
                let lv = elems[0].trim();
                if lv.len() > 1 {
                    return Err(Box::new(LParseErr::UnexpectedValue(
                        self.cur_line,
                        "<одиночный символ>".into(),
                        lv.into(),
                    )));
                }
                let lv = lv.chars().next().unwrap();

                // <строка над алфавитом> | <операция>
                let rv = elems[1].trim();
                if rv.is_empty() {
                    return Err(Box::new(LParseErr::UnexpectedValue(
                        self.cur_line,
                        "<строка над алфавитом> | <операция>".into(),
                        "''".into(),
                    )));
                }

                // <операция>
                if op_keywords.iter().any(|keyword| rv.contains(*keyword)) {
                    if rv.starts_with("FORWARD") {
                        // <символ> -> FORWARD RANDOM <начало диапазона> <конец диапазона>
                        if rv.contains("RANDOM") {
                            let rv_trimmed = rv
                                .trim_start_matches("FORWARD")
                                .trim()
                                .trim_start_matches("RANDOM")
                                .trim();
                            let rv_nums: Vec<&str> = rv_trimmed.split_whitespace().collect();
                            if rv_nums.len() != 2 {
                                return Err(Box::new(LParseErr::UnexpectedValue(
                                    self.cur_line,
                                    "<начало диапазона> <конец диапазона>".into(),
                                    rv_trimmed.into(),
                                )));
                            }
                            let range_begin: f32 = rv_nums[0].parse()?;
                            let range_end: f32 = rv_nums[1].parse()?;
                            actions.insert(lv, Actions::ForwardRandom(range_begin, range_end));
                        }
                        // <символ> -> FORWARD <длина>
                        else {
                            let rv_trimmed = rv.trim_start_matches("FORWARD").trim();
                            if rv_trimmed.is_empty() {
                                return Err(Box::new(LParseErr::UnexpectedValue(
                                    self.cur_line,
                                    "<длина>".into(),
                                    "''".into(),
                                )));
                            }
                            let length: f32 = rv_trimmed.parse()?;
                            actions.insert(lv, Actions::Forward(length));
                        }
                    } else if rv.starts_with("ROTATE") {
                        // <символ> -> ROTATE RANDOM (- | +) <отклонение>
                        if rv.contains("RANDOM") {
                            let mut rv_trimmed = rv
                                .trim_start_matches("ROTATE")
                                .trim()
                                .trim_start_matches("RANDOM")
                                .trim();
                            if rv_trimmed.is_empty() {
                                return Err(Box::new(LParseErr::UnexpectedValue(
                                    self.cur_line,
                                    "(- | + ) <отклонение>".into(),
                                    "''".into(),
                                )));
                            }
                            let mut sign = 1.0;
                            if rv_trimmed.contains('-') {
                                sign = -1.0;
                                rv_trimmed = rv_trimmed.trim_start_matches("-").trim();
                            } else {
                                rv_trimmed = rv_trimmed.trim_start_matches("+").trim();
                            }

                            if rv_trimmed.is_empty() {
                                return Err(Box::new(LParseErr::UnexpectedValue(
                                    self.cur_line,
                                    "<отклонение>".into(),
                                    "''".into(),
                                )));
                            }

                            let delta: f32 = rv_trimmed.parse()?;
                            actions.insert(
                                lv,
                                Actions::RotateRandom(
                                    sign * rotate_angle - delta,
                                    sign * rotate_angle + delta,
                                ),
                            );
                        }
                        // <символ> -> ROTATE (+ | -)
                        else {
                            let rv_trimmed = rv.trim_start_matches("ROTATE").trim();
                            if rv_trimmed.len() != 1 {
                                return Err(Box::new(LParseErr::UnexpectedValue(
                                    self.cur_line,
                                    "+ | -".into(),
                                    rv_trimmed.into(),
                                )));
                            }
                            let mut angle = rotate_angle;
                            if rv_trimmed.contains('-') {
                                angle = -angle;
                            }
                            actions.insert(lv, Actions::Rotate(angle));
                        }
                    }
                    // <символ> -> SAVE
                    else if rv.starts_with("SAVE") {
                        let rv_trimmed = rv.trim_start_matches("SAVE").trim();
                        if !rv_trimmed.is_empty() {
                            return Err(Box::new(LParseErr::UnexpectedValue(
                                self.cur_line,
                                "''".into(),
                                rv_trimmed.into(),
                            )));
                        }
                        actions.insert(lv, Actions::Save);
                    }
                    // <символ> -> RESTORE
                    else if rv.starts_with("RESTORE") {
                        let rv_trimmed = rv.trim_start_matches("RESTORE").trim();
                        if !rv_trimmed.is_empty() {
                            return Err(Box::new(LParseErr::UnexpectedValue(
                                self.cur_line,
                                "''".into(),
                                rv_trimmed.into(),
                            )));
                        }
                        actions.insert(lv, Actions::Restore);
                    } else {
                        panic!("Ключевое слово для действия найдено, но почему-то не обработано");
                    }
                }
                // <строка над алфавитом>
                else {
                    // <символ> -> <строка над алфавитом> <- но самой проверки алфавита не будет, у меня лапки
                    rules.insert(lv, rv.into());
                }
            }
            // <присвоение>
            else if line.contains("=") {
                let elems: Vec<&str> = line.split("=").collect();
                if elems.len() != 2 {
                    return Err(Box::new(LParseErr::UnexpectedValuesAmount(
                        self.cur_line,
                        3,
                        elems.len(),
                    )));
                }

                // <переменная>
                let lv = elems[0].trim();
                if !var_keywords.iter().any(|keyword| lv == *keyword) {
                    return Err(Box::new(LParseErr::UnexpectedValue(
                        self.cur_line,
                        "WIDTH | WIDTH_DELTA | COLOR | COLOR_DELTA".into(),
                        lv.into(),
                    )));
                }

                // <значение>
                let rv = elems[1].trim();

                if lv == "WIDTH" || lv == "WIDTH_DELTA" {
                    let value: f32 = rv.parse()?;
                    // WIDTH = <float32>
                    if lv == "WIDTH" {
                        width = value;
                    }
                    // WIDTH_DELTA = <float32>
                    else {
                        width_delta = value;
                    }
                } else if lv == "COLOR" || lv == "COLOR_DELTA" {
                    let rgb: Vec<&str> = rv.get(1..rv.len() - 1).unwrap().split(",").collect();
                    let value = egui::Color32::from_rgb(
                        rgb[0]
                            .trim()
                            .parse()
                            .expect(&format!("invalid red: {}", rgb[0])),
                        rgb[1]
                            .trim()
                            .parse()
                            .expect(&format!("invalid green: {}", rgb[0])),
                        rgb[2]
                            .trim()
                            .parse()
                            .expect(&format!("invalid blue: {}", rgb[0])),
                    );
                    if lv == "COLOR" {
                        color = value;
                    } else {
                        color_delta = value;
                    }
                } else {
                    panic!("Обнаружено присвоение, но неизвестная переменная");
                }
            } else {
                return Err(Box::new(LParseErr::UnexpectedValue(
                    self.cur_line,
                    "<правило> | <действие> | <присвоение>".into(),
                    line.into(),
                )));
            }
            line.clear();
        }

        Ok(LSystemConfig {
            axiom,
            rules,
            actions,
            width,
            width_delta,
            angle,
            color,
            color_delta,
        })
    }
}
