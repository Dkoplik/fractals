use crate::app::logic::transform2d::Transform2D;
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

/// Линия, нарисованная L-системой
#[derive(Debug, Clone)]
struct Line {
    /// Начало линии (отрезка).
    begin: egui::Pos2,
    /// Конец линии (отрезка).
    end: egui::Pos2,
    /// Ширина линии (отрезка).
    width: f32,
    /// Цвет линии (отрезка).
    color: egui::Color32,
}

impl Line {
    fn draw(&self, painter: &egui::Painter) {
        painter.line(
            vec![self.begin, self.end],
            egui::epaint::PathStroke::new(self.width, self.color),
        );
    }
}

/// Текущая L-система с конфигурацией и итерацией.
pub struct Lsystem {
    /// Конфигурация.
    config: LSystemConfig,
    /// Текущая строка.
    cur_string: String,
    /// Текущая итерация.
    iter: u32,
    /// Текущие линии (изображение).
    lines: Vec<Line>,
}

impl Lsystem {
    pub fn new(config: LSystemConfig) -> Self {
        Self {
            cur_string: config.axiom.clone(),
            iter: 0,
            config,
            lines: Vec::new(), // TODO заполнить линиями из аксиомы
        }
    }

    /// Провести ещё одну итерацию L-системы.
    pub fn iter_once() {
        // TODO cur_string - текущая итерация, над ней сделать ещё одну итерацию и построить линии в lines

        /// Структура ниже в помощь для операций Save и Restore на стеке.
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
    }

    fn find_rect(&self) -> Option<egui::Rect> {
        let mut pos_min: Option<egui::Pos2> = None;
        let mut pos_max: Option<egui::Pos2> = None;
        for line in &self.lines {
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

        if let Some(pos_min) = pos_min && let Some(pos_max) = pos_max {
            Some(egui::Rect::from_min_max(pos_min, pos_max))
        } else {
            None
        }
    }

    fn draw(&self, painter: &egui::Painter, area: egui::Rect, margin: f32) {
        let sys_rect = self.find_rect();
        if sys_rect.is_none() {
            return;
        }
        let mut sys_rect = sys_rect.unwrap();

        // scale image
        let scale = (area.width() / sys_rect.width()).min(area.height() / sys_rect.height());
        let transform = Transform2D::uniform_scaling(scale);
        sys_rect.min = transform.apply_to_pos(sys_rect.min);
        sys_rect.max = transform.apply_to_pos(sys_rect.max);

        // center image
        let delta_vec = area.center() - sys_rect.center();
        transform = transform.multiply();


        self.lines.iter().cloned()
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

/// Простой парсинг файла с конфигурацией L-системы
pub fn parse_l_system(
    file_path: impl AsRef<Path>,
) -> Result<LSystemConfig, Box<dyn std::error::Error>> {
    let op_keywords = vec!["FORWARD", "ROTATE", "SAVE", "RESTORE"];
    let var_keywords = vec!["WIDTH", "WIDTH_DELTA", "COLOR", "COLOR_DELTA"];

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut cur_line: usize = 0;

    // <аксиома> <угол поворота в градусах> <начальное направление в градусах>
    let mut line = String::new();
    reader.read_line(&mut line)?;
    cur_line += 1;
    let first_line_elems: Vec<&str> = line.split_whitespace().collect();
    if first_line_elems.len() != 3 {
        return Err(Box::new(LParseErr::UnexpectedValuesAmount(
            cur_line,
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
    let mut width = 5.0;
    let mut width_delta = 0.0;
    let mut color = egui::Color32::BLACK;
    let mut color_delta = egui::Color32::from_rgb(0, 0, 0);
    line.clear();
    while reader.read_line(&mut line)? > 0 {
        cur_line += 1;

        // <символ> -> <строка над алфавитом> | <символ> -> <операция>
        if line.contains("->") {
            let elems: Vec<&str> = line.split("->").collect();
            if elems.len() != 2 {
                return Err(Box::new(LParseErr::UnexpectedValuesAmount(
                    cur_line,
                    3,
                    elems.len(),
                )));
            }

            // <символ>
            let lv = elems[0];
            if lv.len() > 1 {
                return Err(Box::new(LParseErr::UnexpectedValue(
                    cur_line,
                    "<одиночный символ>".into(),
                    lv.into(),
                )));
            }
            let lv = lv.chars().next().unwrap();

            // <строка над алфавитом> | <операция>
            let rv = elems[1].trim();
            if rv.is_empty() {
                return Err(Box::new(LParseErr::UnexpectedValue(
                    cur_line,
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
                                cur_line,
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
                                cur_line,
                                "<длина>".into(),
                                "''".into(),
                            )));
                        }
                        let length: f32 = rv_trimmed.parse()?;
                        actions.insert(lv, Actions::Forward(length));
                    }
                } else if rv.starts_with("ROTATE") {
                    // <символ> -> ROTATE RANDOM <отклонение>
                    if rv.contains("RANDOM") {
                        let rv_trimmed = rv
                            .trim_start_matches("ROTATE")
                            .trim()
                            .trim_start_matches("RANDOM")
                            .trim();
                        if rv_trimmed.is_empty() {
                            return Err(Box::new(LParseErr::UnexpectedValue(
                                cur_line,
                                "<отклонение>".into(),
                                "''".into(),
                            )));
                        }
                        let delta: f32 = rv_trimmed.parse()?;
                        actions.insert(
                            lv,
                            Actions::RotateRandom(rotate_angle - delta, rotate_angle + delta),
                        );
                    }
                    // <символ> -> ROTATE
                    else {
                        let rv_trimmed = rv.trim_start_matches("ROTATE").trim();
                        if !rv_trimmed.is_empty() {
                            return Err(Box::new(LParseErr::UnexpectedValue(
                                cur_line,
                                "''".into(),
                                rv_trimmed.into(),
                            )));
                        }
                        actions.insert(lv, Actions::Rotate(rotate_angle));
                    }
                }
                // <символ> -> SAVE
                else if rv.starts_with("SAVE") {
                    let rv_trimmed = rv.trim_start_matches("SAVE").trim();
                    if !rv_trimmed.is_empty() {
                        return Err(Box::new(LParseErr::UnexpectedValue(
                            cur_line,
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
                            cur_line,
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
                    cur_line,
                    3,
                    elems.len(),
                )));
            }

            // <переменная>
            let lv = elems[0].trim();
            if !var_keywords.iter().any(|keyword| lv == *keyword) {
                return Err(Box::new(LParseErr::UnexpectedValue(
                    cur_line,
                    "WIDTH | WIDTH_DELTA | COLOR | COLOR_DELTA".into(),
                    lv.into(),
                )));
            }

            // <значение>
            let rv = elems[1].trim();

            if lv == "WIDTH" || lv == "WIDTH_DELTA" {
                let value: f32 = rv.parse()?;
                // WIDTH = <float23>
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
                    rgb[0].trim().parse()?,
                    rgb[1].trim().parse()?,
                    rgb[2].trim().parse()?,
                );
                if lv == "WIDTH" {
                    color = value;
                } else {
                    color_delta = value;
                }
            } else {
                panic!("Обнаружено присвоение, но неизвестная переменная");
            }
        } else {
            return Err(Box::new(LParseErr::UnexpectedValue(
                cur_line,
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
