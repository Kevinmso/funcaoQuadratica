use eframe::{
    egui::{
        self,
        Color32, Ui,
    },
    App,
};

use egui_plot::{
    Plot,
    PlotPoints,
    Line,
    Points,
};

use std::ops::Range;
use crate::matematica::Quadratica;

pub struct AppUI {
    funcao: Quadratica,
    intervalo: Range<f64>,.
    passos: usize,
}

impl Default for AppUI {
    fn default() -> Self {
        Self {
            funcao: Quadratica::new(1.0, -1.0, 0.0),
            intervalo: -5.0..5.0,
            passos: 500,
        }
    }
}

impl App for AppUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gráfico de Função Quadrática");
            
            // Controles para os coeficientes
            ui.horizontal(|ui| {
                ui.label("Coeficientes:");
                ui.add(egui::DragValue::new(&mut self.funcao.a).speed(0.1).prefix("a: "));
                ui.add(egui::DragValue::new(&mut self.funcao.b).speed(0.1).prefix("b: "));
                ui.add(egui::DragValue::new(&mut self.funcao.c).speed(0.1).prefix("c: "));
            });

            // Intervalo do gráfico
            ui.horizontal(|ui| {
                ui.label("Intervalo:");
                ui.add(egui::DragValue::new(&mut self.intervalo.start).speed(0.1).prefix("min: "));
                ui.add(egui::DragValue::new(&mut self.intervalo.end).speed(0.1).prefix("max: "));
            });

            // Informações importantes
            ui.separator();
            let raizes = self.funcao.calcular_raizes();
            let vertice = self.funcao.calcular_vertice();
            let intercepto_y = self.funcao.calcular_intercepto_y();
            
            ui.label(format!("Vértice: ({:.2}, {:.2})", vertice.0, vertice.1));
            ui.label(format!("Intercepto Y: ({:.2}, {:.2})", intercepto_y.0, intercepto_y.1));
            ui.label(format!("Δ (Delta): {:.2}", self.funcao.calcular_delta()));
            
            if !raizes.is_empty() {
                ui.label(format!("Raízes: {}", raizes
                    .iter()
                    .map(|r| format!("{:.2}", r))
                    .collect::<Vec<_>>()
                    .join(", ")));
            } else {
                ui.label("Não há raízes reais");
            }

            // Plot do gráfico
            Plot::new("grafico_quadratica")
                .height(400.0)
                .show(ui, |plot_ui| {
                    // Plotar a função
                    let pontos = self.funcao.gerar_pontos(self.intervalo.clone(), self.passos)
                        .into_iter()
                        .map(|(x, y)| [x, y]) // Convertendo (f64, f64) para [f64; 2]
                        .collect::<Vec<_>>();
                    let linha = Line::new("f(x)", PlotPoints::from_iter(pontos))
                        .color(Color32::from_rgb(100, 200, 100));
                    plot_ui.line(linha);

                    // Plotar pontos importantes
                    let pontos_importantes = vec![vertice, intercepto_y]
                        .into_iter()
                        .chain(raizes.into_iter().map(|r| (r, 0.0)))
                        .map(|(x, y)| [x, y]) // Convertendo (f64, f64) para [f64; 2]
                        .collect::<Vec<_>>();

                    let pontos_plot = Points::new("Pontos Importantes", PlotPoints::from_iter(pontos_importantes))
                        .color(Color32::RED)
                        .radius(5.0);
                    plot_ui.points(pontos_plot);
                });
        });
    }
}


pub fn criar_app_com_valores(a: f64, b: f64, c: f64) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    // Criar uma instância personalizada de AppUI com os valores fornecidos
    let app = AppUI {
        funcao: Quadratica::new(a, b, c),
        intervalo: -5.0..5.0,
        passos: 500,
    };

    eframe::run_native(
        "Função Quadrática",
        options,
        Box::new(move |_cc| Ok(Box::new(app))),
    )
}