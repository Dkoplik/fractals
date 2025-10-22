use egui::Label;

use crate::app::FractalsApp;

// --------------------------------------------------
// Построение UI приложения
// --------------------------------------------------

impl eframe::App for FractalsApp {
    /// Главный цикл UI.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_top_panel(ctx);
        self.show_left_panel(ctx);
        self.show_bottom_panel(ctx);
        self.show_cental_panel(ctx);
    }
}

impl FractalsApp {
    /// Показать верхную панель приложения.
    fn show_top_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });
    }

    /// Показать левую панель приложения.
    fn show_left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label("Фрактальный тип:");

                    // Выбор типа фрактала
                    ui.horizontal(|ui| {
                        if ui.button("L-системы").clicked() {
                            self.fractal_type = crate::app::FractalType::LSystem;
                        }
                        if ui.button("Горы").clicked() {
                            self.fractal_type = crate::app::FractalType::MidpointDisplacement;
                        }
                        if ui.button("Сплайны").clicked() {
                            self.fractal_type = crate::app::FractalType::BezierSpline;
                        }
                    });

                    ui.separator();

                    // Общие настройки
                    ui.label("Общие настройки:");
                    if ui.button("Очистить холст").clicked() {
                        self.clear_canvas();
                    }

                    ui.separator();

                    // Настройки для L-систем
                    if self.fractal_type == crate::app::FractalType::LSystem {
                        ui.label("L-системы:");
                        if ui.button("Загрузить L-систему").clicked() {
                            self.load_lsystem();
                        }

                        if ui.button("Следующая итерация").clicked() {
                            self.iterate_fractal();
                        }

                        if let Some(l) = &self.lsystem {
                            ui.label(l.to_string());
                        }
                    }

                    // Настройки для Midpoint Displacement
                    if self.fractal_type == crate::app::FractalType::MidpointDisplacement {
                        ui.label("Алгоритм Midpoint Displacement:");
                        
                        ui.horizontal(|ui| {
                            if ui.button("Сгенерировать").clicked() {
                                self.generate_mountains();
                            }
                            if ui.button("Следующая итерация").clicked() {
                                self.iterate_fractal();
                            }
                        });

                        ui.separator();
                        
                        ui.label("Параметры генерации:");
                        ui.add(
                            egui::Slider::new(&mut self.md_roughness, 0.1..=2.0)
                                .text("Шероховатость")
                        );
                        ui.add(
                            egui::Slider::new(&mut self.md_iterations, 1..=12)
                                .text("Начальные итерации")
                        );

                        ui.checkbox(&mut self.md_show_steps, "Показывать информацию");
                        
                        ui.separator();
                        
                        // Информация о текущем состоянии
                        ui.label(format!("Текущая итерация: {}", self.midpoint_displacement.cur_iter_num()));
                    }

                    // Настройки для сплайнов Безье
                    if self.fractal_type == crate::app::FractalType::BezierSpline {
                        ui.label("Сплайны Безье:");

                        ui.horizontal(|ui| {
                            if ui.button("Добавить точку").clicked() {
                                self.instrument = crate::app::Instrument::AddPoint;
                            }
                            if ui.button("Удалить точку").clicked() {
                                self.instrument = crate::app::Instrument::RemovePoint;
                            }
                        });

                        ui.horizontal(|ui| {
                            if ui.button("Перемещать").clicked() {
                                self.instrument = crate::app::Instrument::MovePoint;
                            }
                        });
                    }
                });
            });
    }

    /// Показать нижнюю панель приложения.
    fn show_bottom_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Тип фрактала: {}", self.fractal_type));
                ui.separator();
                ui.label(format!("Инструмент: {}", self.instrument));
                ui.separator();
                ui.label(format!(
                    "Размер холста: {:.1} x {:.1}",
                    self.painter_width, self.painter_height
                ));
                ui.separator();
                ui.label(format!("Количество точек: {}", self.point_count));
                ui.separator();
                ui.label(format!("Итерация: {}", self.current_iteration));
            });
        });
    }

    /// Показать центральную (основную) панель приложения.
    fn show_cental_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Resize::default()
                .default_size(egui::Vec2 { x: 900.0, y: 600.0 })
                .show(ui, |ui| {
                    // Выделяем область для рисования
                    let (response, painter) = self.allocate_painter(ui);

                    // Рисуем текущий фрактал
                    self.draw_canvas(&painter);

                    // Обрабатываем ввод
                    self.handle_input(&response);

                    // Показываем подсказку в зависимости от выбранного инструмента
                    if self.fractal_type == crate::app::FractalType::BezierSpline {
                        let hint = match self.instrument {
                            crate::app::Instrument::AddPoint => "Кликните для добавления точки",
                            crate::app::Instrument::RemovePoint => "Кликните на точку для удаления",
                            crate::app::Instrument::MovePoint => "Кликните и перетащите точку",
                            crate::app::Instrument::None => "Выберите инструмент на левой панели",
                        };

                        painter.text(
                            response.rect.left_bottom() + egui::Vec2::new(10.0, -10.0),
                            egui::Align2::LEFT_BOTTOM,
                            hint,
                            egui::FontId::default(),
                            egui::Color32::GRAY,
                        );
                    }
                });
        });
    }
}
