#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

enum ContractType {
    Daily,
    Weekly,
    Monthly,
}

fn contract_type_to_string(contract_type: &Option<ContractType>) -> String {
    match contract_type {
        Some(ContractType::Daily) => "Diário".to_string(),
        Some(ContractType::Weekly) => "Semanal".to_string(),
        Some(ContractType::Monthly) => "Mensal".to_string(),
        None => "Nenhum".to_string(),
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Registro de Lucros e Gastos - Empilhadeiras",
        options,
        Box::new(|cc| {
            Box::<MyApp>::default()
        }),
    )
}

struct Entry {
    name: String,
    profit: f32,
    expenses: f32,
    contract_type: Option<ContractType>,
}

impl Entry {
    fn balance(&self) -> f32 {
        self.profit - self.expenses
    }
}

struct MyApp {
    name: String,
    profit: String,
    expenses: String,
    daily_checked: bool,
    weekly_checked: bool,
    monthly_checked: bool,
    entries: Vec<Entry>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: String::new(),
            profit: String::new(),
            expenses: String::new(),
            daily_checked: false,
            weekly_checked: false,
            monthly_checked: false,
            entries: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Registro de Lucros e Gastos - Empilhadeiras");

            ui.horizontal(|ui| {
                let name_label = ui.label("Nome da Empilhadeira: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });

            ui.horizontal(|ui| {
                ui.label("Lucro: R$ ");
                ui.add(egui::widgets::TextEdit::singleline(&mut self.profit));
            });

            ui.horizontal(|ui| {
                ui.label("Gastos: R$ ");
                ui.add(egui::widgets::TextEdit::singleline(&mut self.expenses));
            });

            ui.horizontal(|ui| {
                if ui.checkbox(&mut self.daily_checked, "Diário").clicked() {
                    self.daily_checked = true;
                    self.weekly_checked = false;
                    self.monthly_checked = false;
                }
                if ui.checkbox(&mut self.weekly_checked, "Semanal").clicked() {
                    self.daily_checked = false;
                    self.weekly_checked = true;
                    self.monthly_checked = false;
                }
                if ui.checkbox(&mut self.monthly_checked, "Mensal").clicked() {
                    self.daily_checked = false;
                    self.weekly_checked = false;
                    self.monthly_checked = true;
                }
            });

            if ui.button("Registrar").clicked() {
                if let (Ok(profit), Ok(expenses)) = (self.profit.parse::<f32>(), self.expenses.parse::<f32>()) {
                    let contract_type = if self.daily_checked {
                        Some(ContractType::Daily)
                    } else if self.weekly_checked {
                        Some(ContractType::Weekly)
                    } else if self.monthly_checked {
                        Some(ContractType::Monthly)
                    } else {
                        None
                    };

                    let entry = Entry {
                        name: self.name.clone(),
                        profit,
                        expenses,
                        contract_type,
                    };
                    self.entries.push(entry);

                    // Limpar campos após o registro
                    self.name.clear();
                    self.profit.clear();
                    self.expenses.clear();
                    self.daily_checked = false;
                    self.weekly_checked = false;
                    self.monthly_checked = false;
                } else {
                    // Se a conversão falhar, você pode lidar com isso aqui
                }
            }

            ui.separator();

            ui.heading("Registros:");

            ui.horizontal(|ui| {
                ui.label("Empilhadeira");
                ui.label("Lucro");
                ui.label("Gastos");
                ui.label("Saldo");
                ui.label("Tipo de Contrato");
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                for entry in &self.entries {
                    ui.horizontal(|ui| {
                        ui.label(&entry.name);
                        ui.label(&format!("R$ {:.2}", entry.profit));
                        ui.label(&format!("R$ {:.2}", entry.expenses));
                        ui.label(&format!("R$ {:.2}", entry.balance()));
                        ui.label(&contract_type_to_string(&entry.contract_type));
                    });
                }
            });
        });
    }
}
