use chrono::{NaiveDate, Local};
use eframe::egui;

enum ContractType {
    Daily { days_rented: u32, registration_date: NaiveDate },
    None,
}

fn contract_type_to_string(contract_type: &ContractType) -> String {
    match contract_type {
        ContractType::Daily { days_rented, registration_date } => {
            format!("{} dias | {}", days_rented, registration_date)
        }
        ContractType::None => "Nenhum".to_string(),
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
    contract_type: ContractType,
    description: String,
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
    days_rented: String,
    description: String,
    entries: Vec<Entry>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: String::new(),
            profit: String::new(),
            expenses: String::new(),
            days_rented: String::new(),
            description: String::new(),
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
                ui.label("Dias Alugada: ");
                ui.add(egui::widgets::TextEdit::singleline(&mut self.days_rented));
            });

            ui.horizontal(|ui| {
                ui.label("Descrição: ");
                ui.add(egui::widgets::TextEdit::singleline(&mut self.description));
            });

            if ui.button("Registrar").clicked() {
                if let (Ok(profit), Ok(expenses), Ok(days_rented)) = (
                    self.profit.parse::<f32>(),
                    self.expenses.parse::<f32>(),
                    self.days_rented.parse::<u32>(),
                ) {
                    let contract_type = if days_rented > 0 {
                        ContractType::Daily {
                            days_rented,
                            registration_date: Local::today().naive_local(),
                        }
                    } else {
                        ContractType::None
                    };

                    let entry = Entry {
                        name: self.name.clone(),
                        profit,
                        expenses,
                        contract_type,
                        description: self.description.clone(),
                    };
                    self.entries.push(entry);

                    // Limpar campos após o registro
                    self.name.clear();
                    self.profit.clear();
                    self.expenses.clear();
                    self.days_rented.clear();
                    self.description.clear();
                } else {
                    // Se a conversão falhar, você pode lidar com isso aqui
                }
            }

            ui.separator();

            ui.heading("Registros:");

            let mut total_balance = 0.0;
            ui.horizontal(|ui| {
                ui.label("Empilhadeira");
                ui.label("Lucro");
                ui.label("Gastos");
                ui.label("Saldo");
                ui.label("Dias");
                ui.label("Descrição");
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                for entry in &self.entries {
                    ui.horizontal(|ui| {
                        ui.label(&entry.name);
                        ui.label(&format!("R$ {:.2}", entry.profit));
                        ui.label(&format!("R$ {:.2}", entry.expenses));
                        let balance = entry.balance();
                        total_balance += balance;
                        ui.label(&format!("R$ {:.2}", balance));
                        ui.label(&contract_type_to_string(&entry.contract_type));
                        ui.label(&entry.description);
                    });
                }
            });

            // Exibir o valor total de saldo
            ui.label(format!("Rendimento Total: R$ {:.2}", total_balance));
        });
    }
}
