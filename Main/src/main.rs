#![windows_subsystem = "windows"]
use chrono::{NaiveDate, Local};
use eframe::egui;
use nfd::Response;
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

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
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 600.0]), // Tamanho da janela
        ..Default::default()
    };
    eframe::run_native(
        "WWEmpilhadeiras",
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
                    .labelled_by(name_label.id)
                    // .desired_width(200.0);
            });

            ui.horizontal(|ui| {
                ui.label("Lucro: R$ ");
                ui.add(egui::widgets::TextEdit::singleline(&mut self.profit).desired_width(100.0));
            });

            ui.horizontal(|ui| {
                ui.label("Gastos: R$ ");
                ui.add(egui::widgets::TextEdit::singleline(&mut self.expenses).desired_width(100.0));
            });

            ui.horizontal(|ui| {
                ui.label("Dias Alugada: ");
                ui.add(egui::widgets::TextEdit::singleline(&mut self.days_rented).desired_width(50.0));
            });

            ui.horizontal(|ui| {
                ui.label("Descrição: ");
                ui.add(egui::widgets::TextEdit::singleline(&mut self.description).desired_width(200.0));
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
                            registration_date: Local::now().naive_local().date(),
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

            ui.horizontal(|ui| {
                if ui.button("Carregar CSV").clicked() {
                    match nfd::open_file_dialog(Some("csv"), None).unwrap() {
                        Response::Okay(file_path) => {
                            match load_from_csv(&file_path) {
                                Ok(entries) => self.entries = entries,
                                Err(err) => eprintln!("Erro ao carregar o arquivo CSV: {}", err),
                            }
                        }
                        _ => {} // O usuário cancelou a seleção do arquivo
                    }
                }
                if ui.button("Salvar CSV").clicked() {
                    if let Err(err) = save_to_csv(&self.entries) {
                        eprintln!("Erro ao salvar o arquivo CSV: {}", err);
                    }
                }
            });

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
        });
    }
}

fn save_to_csv(entries: &[Entry]) -> Result<(), Box<dyn Error>> {
    let response = nfd::open_pick_folder(None)?;
    match response {
        Response::Okay(folder_path) => {
            let file_name = "Relatorio_Empilhadeira.csv";
            let file_path = format!("{}/{}", folder_path, file_name);
            let mut file = File::create(&file_path)?;

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
        Response::Cancel => {
            // O usuário cancelou a operação
            Ok(())
        }
        _ => Err("Operação não suportada".into()),
    }
}


fn load_from_csv(file_path: &str) -> Result<Vec<Entry>, Box<dyn Error>> {
    let mut entries = Vec::new();
    if Path::new(file_path).exists() {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        for line in reader.lines().skip(1) {
            let line = line?;
            let mut fields = line.split(';').map(|s| s.trim_matches('"')).collect::<Vec<_>>();
            if fields.len() >= 7 {
                let name = fields[0].to_string();
                let profit = fields[1].parse::<f32>().unwrap_or(0.0);
                let expenses = fields[2].parse::<f32>().unwrap_or(0.0);
                let balance = fields[3].parse::<f32>().unwrap_or(0.0);
                let days_rented = fields[4].to_string();
                let registration_date = fields[5].to_string();
                let description = fields[6].to_string();
                let contract_type = if days_rented != "0" {
                    ContractType::Daily {
                        days_rented: days_rented.parse().unwrap_or(0),
                        registration_date: NaiveDate::parse_from_str(&registration_date, "%Y-%m-%d").unwrap_or_else(|_| NaiveDate::from_ymd(1970, 1, 1)),
                    }
                } else {
                    ContractType::None
                };
                entries.push(Entry {
                    name,
                    profit,
                    expenses,
                    contract_type,
                    description,
                });
            }
        }
    }
    Ok(entries)
}
