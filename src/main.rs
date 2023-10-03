#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{Vec2, Rounding, TextEdit, FontId, FontSelection, FontFamily, Button, Color32};
use yarer::session::Session;
use egui::TextStyle::*;

/// macro for button definition
macro_rules! butt {
    ($t:expr) => {
        Button::new($t).min_size(Vec2::new(64.,64.)).rounding(Rounding::same(4.)).fill(Color32::BLACK)
    };
}

struct CalculatorState {
    expression: String,
    session: Session,
    completed_op: bool,
    arc: bool,
}

impl CalculatorState {
    fn new() -> Self {
        CalculatorState { expression: String::new(), session: Session::init(), completed_op : false, arc: false }
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(370.0, 535.0)),
        resizable: false,
        active: true,
        centered: true,
        ..Default::default()
    };

    // Our application state:
    let mut state = CalculatorState::new();

    fn clear_state(state: &mut CalculatorState) {
        if state.completed_op == true {
            state.completed_op = false;
            state.expression.clear();
        }
    }
           
    eframe::run_simple_native("Yarer GUI Calculator", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.style_mut().text_styles = [
                   (Body, FontId::new(24.0, FontFamily::Proportional)),
                   (Button, FontId::new(24.0, FontFamily::Proportional)),
                 ].into();

            ctx.style_mut(|style| {
                style.visuals.code_bg_color = Color32::BLACK;
                style.visuals.window_fill = Color32::BLACK;
                style.visuals.panel_fill = Color32::BLACK;
            });
    
            let font = FontSelection::FontId(FontId {
                size: 24.0,
                family: FontFamily::Monospace,
            });
            let expression_text = TextEdit::singleline(&mut state.expression)
                .horizontal_align(egui::Align::Max)
                .font(font);

            ui.visuals_mut().panel_fill = Color32::BLACK;
            ui.visuals_mut().window_fill = Color32::BLACK;

            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                ui.add(expression_text)
            });
               
            ui.separator();

            const ORANGE: Color32 = Color32::from_rgb(255, 165, 0);

            ui.visuals_mut().override_text_color = Some(Color32::GRAY);
            
            let sin_b : String = String::from(if state.arc == true { "sin" } else { "asin"});
            let cos_b : String = String::from(if state.arc == true { "cos" } else { "acos"});
            let tan_b : String = String::from(if state.arc == true { "tan" } else { "atan"});

            let b_inv = butt!("INV");
            let b_abs = butt!("abs");
            let b_sin = butt!(sin_b);
            let b_cos = butt!(cos_b);
            let b_tan = butt!(tan_b);
            ui.horizontal(|ui| {
                if ui.add(b_inv).clicked() {
                    state.arc = !state.arc;
                }
                if ui.add(b_abs).clicked() {
                    state.expression += "abs(";
                }
                if ui.add(b_sin).clicked() {
                    state.expression += if state.arc == true { "sin(" } else { "asin("};
                }
                if ui.add(b_cos).clicked() {
                    state.expression += if state.arc == true { "cos(" } else { "acos("};
                }
                if ui.add(b_tan).clicked() {
                    state.expression += if state.arc == true { "tan(" } else { "atan("};
                }
            });

            let b_pow = butt!("x^y");
            let b_log = butt!("log");
            let b_ln = butt!("ln");
            let b_open_bracket = butt!("(");
            let b_close_bracket = butt!(")");
            ui.horizontal(|ui| {
                if ui.add(b_pow).clicked() {
                    state.expression += "^";
                }
                if ui.add(b_log).clicked() {
                    state.expression += "log(";
                }
                if ui.add(b_ln).clicked() {
                    state.expression += "ln(";
                }
                if ui.add(b_open_bracket).clicked() {
                    state.expression += "(";
                }
                if ui.add(b_close_bracket).clicked() {
                    state.expression += ")";
                }
            
            });

            let b_sqrt = butt!("√x");
            let b_ca = butt!("C");
            let b_back = butt!("Del");
            let b_percentage = butt!("%");
            let b_div = butt!("/");
            ui.horizontal(|ui| {
                if ui.add(b_sqrt).clicked() {
                    state.expression += "^";
                }
                ui.visuals_mut().override_text_color = Some(ORANGE);
                if ui.add(b_ca).clicked() {
                    state.expression.clear();
                }
                if ui.add(b_back).clicked() {
                    let len = state.expression.len();
                    if len != 0 {
                        state.expression.remove(state.expression.len()-1);
                    }
                }
                if ui.add(b_percentage).clicked() {
                    state.expression += "%";
                }
                if ui.add(b_div).clicked() {
                    state.completed_op = false;
                    state.expression += "/";
                }
            });
        
            
            let b_fac = butt!("x!");
            let b7 = butt!("7");
            let b8 = butt!("8");
            let b9 = butt!("9");
            let b_mul = butt!("X");
            ui.horizontal(|ui| {
                if ui.add(b_fac).clicked() {
                    state.expression += "!";
                }
                ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                if ui.add(b7).clicked() {
                    clear_state(&mut state);
                    state.expression += "7";
                }
                if ui.add(b8).clicked() {
                    clear_state(&mut state);
                    state.expression += "8";
                }
                if ui.add(b9).clicked() {
                    clear_state(&mut state);
                    state.expression += "9";
                }
                ui.visuals_mut().override_text_color = Some(ORANGE);
                if ui.add(b_mul).clicked() {
                    state.completed_op = false;
                    state.expression += "*";
                }
                ui.visuals_mut().override_text_color = Some(Color32::GRAY);
            });

            let b_rec = butt!("1/x");
            let b4 = butt!("4");
            let b5 = butt!("5");
            let b6 = butt!("6");
            let b_min = butt!("—");
            ui.horizontal(|ui| {
                if ui.add(b_rec).clicked() {
                    state.expression += "^-1";
                }
                ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                if ui.add(b4).clicked() {
                    clear_state(&mut state);
                    state.expression += "4";
                }
                if ui.add(b5).clicked() {
                    clear_state(&mut state);
                    state.expression += "5";
                }
                if ui.add(b6).clicked() {
                    clear_state(&mut state);
                    state.expression += "6";
                }
                ui.visuals_mut().override_text_color = Some(ORANGE);
                if ui.add(b_min).clicked() {
                    state.completed_op = false;
                    state.expression += "-";
                }
                ui.visuals_mut().override_text_color = Some(Color32::GRAY);
            });


            let b1 = butt!("1");
            let b2 = butt!("2");
            let b3 = butt!("3");
            let b_pi = butt!("π");
            let b_add = butt!("➕");
            ui.horizontal(|ui| {
                if ui.add(b_pi).clicked() {
                    state.expression += "pi";
                }
                ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                if ui.add(b1).clicked() {
                    clear_state(&mut state);
                    state.expression += "1";
                }
                if ui.add(b2).clicked() {
                    clear_state(&mut state);
                    state.expression += "2";
                }
                if ui.add(b3).clicked() {
                    clear_state(&mut state);
                    state.expression += "3";
                }
                ui.visuals_mut().override_text_color = Some(ORANGE);
                if ui.add(b_add).clicked() {
                    state.completed_op = false;
                    state.expression += "+";
                }
                ui.visuals_mut().override_text_color = Some(Color32::GRAY);
            });

            let dummy = butt!("").frame(false);
            let b0 = butt!("0");
            let b_dot = butt!(".");
            let b_e = butt!("e");

            let b_res = butt!("=");
            ui.horizontal(|ui| {
                ui.add(dummy);
            
                if ui.add(b_e).clicked() {
                    state.expression += "e";
                }
                ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                if ui.add(b0).clicked() {
                    state.expression += "0";
                }
                ui.visuals_mut().override_text_color = Some(Color32::GRAY);
                if ui.add(b_dot).clicked() {
                    state.expression += ".";
                }
                ui.visuals_mut().override_text_color = Some(ORANGE);
                if ui.add(b_res).clicked() {
                  
                    if state.expression.is_empty() {
                        state.expression += "0";
                    }

                    let mut resolver = state.session.build_resolver_for(&state.expression);
                    
                    match resolver.resolve() {
                        Ok(res) => {
                            state.expression.clear();
                            state.expression += &format!("{}",res)
                        },
                        Err(err) => {
                            eprintln!("* Error: {}", err);
                            state.expression.clear();
                            state.expression += "Error!";
                        },
                    }
                    state.completed_op = true;
                }
            });         
        });
    })
}

