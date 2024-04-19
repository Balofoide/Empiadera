use chrono::{NaiveDate, Local};
use eframe::egui;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

const MAX_PANEL_HEIGHT: f32 = 200.0;

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
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 500.0]), // Definindo o tamanho da janela
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

            let scroll_area = egui::ScrollArea::vertical().max_height(MAX_PANEL_HEIGHT);
            scroll_area.show(ui, |ui| {
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
            ui.separator();
            let total_balance_text = format!("Rendimento Total: R$ {:.2}", total_balance);
            ui.add(egui::Label::new(total_balance_text));

            // Salvando os registros em um arquivo CSV
            if ui.button("Salvar CSV").clicked() {
                if let Err(err) = save_to_csv(&self.entries) {
                    eprintln!("Erro ao salvar o arquivo CSV: {}", err);
                }
            }
        });
    }
}

fn save_to_csv(entries: &[Entry]) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("registros.csv")?;
    writeln!(file, "Nome;Lucro;Gastos;Saldo;Dias;Data;Descricao")?;
    for entry in entries {
        let balance = entry.balance();
        let (days_rented, registration_date) = match &entry.contract_type {
            ContractType::Daily { days_rented, registration_date } => {
                (days_rented.to_string(), registration_date.to_string())
            }
            ContractType::None => ("Nenhum".to_string(), "".to_string()),
        };
        writeln!(
            file,
            "\"{}\";{:.2};{:.2};{:.2};\"{}\";\"{}\";\"{}\"",
            entry.name,
            entry.profit,
            entry.expenses,
            balance,
            days_rented,
            registration_date,
            entry.description
        )?;
    }
    Ok(())
}