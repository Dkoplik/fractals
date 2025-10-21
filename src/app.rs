pub mod logic;
pub mod ui;

use crate::app::logic::bezier;
use crate::app::logic::l_system;
use crate::app::logic::midpoint_displacement;
use egui::Pos2;

// --------------------------------------------------
// Базовое определение приложения
// --------------------------------------------------

#[derive(PartialEq)]
pub enum FractalType {
    LSystem,
    MidpointDisplacement,
    BezierSpline,
}

#[derive(PartialEq)]
pub enum Instrument {
    AddPoint,
    RemovePoint,
    MovePoint,
    None,
}

/// Приложение-демонстрация фракталов.
pub struct FractalsApp {
    // Общие поля
    pub fractal_type: FractalType,
    pub instrument: Instrument,

    // Поля для L-систем
    pub lsystem_iterations: usize,
    pub lsystem_angle: f64,
    pub lsystem_length: f64,
    pub lsystem_randomness: bool,
    pub lsystem_colors: bool,
    pub lsystem_thickness: bool,

    // Поля для Midpoint Displacement
    pub md_roughness: f32,
    pub md_iterations: u32,
    pub md_show_steps: bool,

    // Поля для сплайнов Безье
    pub bezier_segments: usize,
    pub bezier_show_points: bool,
    pub bezier_show_control: bool,

    // Другие необходимые поля
    pub painter_width: f32,
    pub painter_height: f32,
    pub point_count: usize,
    pub current_iteration: usize,

    // Временные данные для разных фракталов
    pub lsystem: Option<l_system::Lsystem>,
    pub midpoint_displacement: midpoint_displacement::MidDisplacement,
    pub bezier_curve: bezier::BezierCurve,

    // Для управления
    pub selected_point: Option<usize>,
    pub drag_prev_pos: Option<Pos2>,
}

impl Default for FractalsApp {
    fn default() -> Self {
        let md_roughness = 1.0;
        Self {
            fractal_type: FractalType::LSystem,
            instrument: Instrument::None,

            // L-системы по умолчанию
            lsystem_iterations: 4,
            lsystem_angle: 25.0,
            lsystem_length: 10.0,
            lsystem_randomness: false,
            lsystem_colors: false,
            lsystem_thickness: false,

            // Midpoint Displacement по умолчанию
            md_roughness,
            md_iterations: 8,
            md_show_steps: false,

            // Сплайны Безье по умолчанию
            bezier_segments: 50,
            bezier_show_points: true,
            bezier_show_control: true,

            // Другие поля
            painter_width: 900.0,
            painter_height: 600.0,
            point_count: 0,
            current_iteration: 0,

            // Временные данные
            lsystem: None,
            midpoint_displacement: midpoint_displacement::MidDisplacement::new(md_roughness),
            // midpoint_displacement: None,
            bezier_curve: bezier::BezierCurve::default(),

            // Для управления
            selected_point: None,
            drag_prev_pos: None,
        }
    }
}

impl FractalsApp {
    /// Инициализация приложения.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // белая тема
        cc.egui_ctx.set_theme(egui::Theme::Light);
        Self::default()
    }
}

impl std::fmt::Display for FractalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FractalType::LSystem => write!(f, "L-системы"),
            FractalType::MidpointDisplacement => write!(f, "Midpoint Displacement"),
            FractalType::BezierSpline => write!(f, "Сплайны Безье"),
        }
    }
}

impl std::fmt::Display for Instrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instrument::AddPoint => write!(f, "Добавить точку"),
            Instrument::RemovePoint => write!(f, "Удалить точку"),
            Instrument::MovePoint => write!(f, "Перемещать точку"),
            Instrument::None => write!(f, "Нет"),
        }
    }
}
