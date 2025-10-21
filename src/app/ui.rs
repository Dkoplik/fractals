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
                            // self.fractal_type = FractalType::LSystem;
                        }
                        if ui.button("Горы").clicked() {
                            // self.fractal_type = FractalType::MidpointDisplacement;
                        }
                        if ui.button("Сплайны").clicked() {
                            // self.fractal_type = FractalType::BezierSpline;
                        }
                    });

                    ui.separator();

                    // Общие настройки
                    ui.label("Общие настройки:");
                    if ui.button("Очистить холст").clicked() {
                        // self.clear_canvas();
                    }

                    if ui.button("Сохранить изображение").clicked() {
                        // self.save_image();
                    }

                    ui.separator();

                    // Настройки для L-систем
                    ui.label("L-системы:");
                    if ui.button("Загрузить L-систему").clicked() {
                        // self.load_lsystem();
                    }

                    if ui.button("Случайное дерево").clicked() {
                        // self.generate_random_tree();
                    }

                    ui.add(
                        egui::Slider::new(&mut self.lsystem_iterations, 1..=10).text("Итерации"),
                    );
                    ui.add(egui::Slider::new(&mut self.lsystem_angle, 0.0..=180.0).text("Угол"));
                    ui.add(egui::Slider::new(&mut self.lsystem_length, 1.0..=100.0).text("Длина"));

                    ui.checkbox(&mut self.lsystem_randomness, "Случайность");
                    ui.checkbox(&mut self.lsystem_colors, "Цвета ветвей");
                    ui.checkbox(&mut self.lsystem_thickness, "Толщина ветвей");

                    ui.separator();

                    // Настройки для Midpoint Displacement
                    ui.label("Алгоритм Midpoint Displacement:");
                    if ui.button("Сгенерировать горы").clicked() {
                        // self.generate_mountains();
                    }

                    ui.add(
                        egui::Slider::new(&mut self.md_roughness, 0.1..=2.0).text("Шероховатость"),
                    );
                    ui.add(egui::Slider::new(&mut self.md_iterations, 1..=12).text("Итерации"));
                    ui.add(egui::Slider::new(&mut self.md_seed, 0..=1000).text("Сид"));

                    ui.checkbox(&mut self.md_show_steps, "Показывать шаги");

                    ui.separator();

                    // Настройки для сплайнов Безье
                    ui.label("Сплайны Безье:");

                    ui.horizontal(|ui| {
                        if ui.button("Добавить точку").clicked() {
                            // self.instrument = Instrument::AddPoint;
                        }
                        if ui.button("Удалить точку").clicked() {
                            // self.instrument = Instrument::RemovePoint;
                        }
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Перемещать").clicked() {
                            // self.instrument = Instrument::MovePoint;
                        }
                        if ui.button("Сбросить").clicked() {
                            // self.reset_bezier();
                        }
                    });

                    ui.add(egui::Slider::new(&mut self.bezier_segments, 10..=100).text("Сегменты"));
                    ui.checkbox(&mut self.bezier_show_points, "Показывать точки");
                    ui.checkbox(
                        &mut self.bezier_show_control,
                        "Показывать контрольные линии",
                    );
                });
            });
    }

    /// Показать нижнюю панель приложения.
    fn show_bottom_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // ui.label(format!("Тип фрактала: {}", self.fractal_type.to_string()));
                // ui.separator();
                // ui.label(format!("Инструмент: {}", self.instrument.to_string()));
                ui.separator();
                ui.label("Размер холста: 900 x 600");
                ui.separator();
                // ui.label(format!("Количество точек: {}", self.point_count));
                // ui.separator();
                // ui.label(format!("Итерация: {}", self.current_iteration));
            });
        });
    }

    /// Показать центральную (основную) панель приложения.
    fn show_cental_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Resize::default()
                .default_size(egui::Vec2 { x: 900.0, y: 600.0 })
                .show(ui, |ui| {
                    // TODO - раскомментировать когда будет готова логика
                    // let (response, painter) = self.allocate_painter(ui);
                    //
                    // match self.fractal_type {
                    //     FractalType::LSystem => self.draw_lsystem(&painter),
                    //     FractalType::MidpointDisplacement => self.draw_mountains(&painter),
                    //     FractalType::BezierSpline => self.draw_bezier(&painter),
                    // }
                    //
                    // self.handle_input(&response);

                    // Временно показываем заглушку
                });
        });
    }
}
