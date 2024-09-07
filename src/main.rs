use arguments::Arguments;
use chrono::NaiveDate;
use clap::Parser;
use colored::Colorize;
use std::cmp::max;
use std::error::Error;
use strum::{EnumIter, IntoEnumIterator};
use table::{dashed_line, table_header};

mod alphabank;
mod arguments;
mod ingbank;
mod table;

#[derive(EnumIter, Debug)]
enum Bank {
    AlphaBank,
    INGBank,
}
impl Bank {
    fn determine_bank(file_name: &str) -> Option<Bank> {
        let file_name = file_name.to_ascii_lowercase();
        Bank::iter().find(|bank| file_name.starts_with(&format!("{:?}", bank).to_ascii_lowercase()))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse();

    let Some(extension) = args.input_file.extension() else {
        return Err("Fișierul nu are extensie.".into());
    };

    if extension != "pdf" {
        return Err("Fișierul nu are extensie PDF.".into());
    }

    let Some(file_name) = args.input_file.file_name() else {
        return Err("Fișierul nu are nume.".into());
    };

    let Some(file_name) = file_name.to_str() else {
        return Err("Fișierul nu are nume valid.".into());
    };

    let Some(bank) = Bank::determine_bank(file_name) else {
        return Err("Fișierul nu are nume de bancă.".into());
    };

    let payment_data = match bank {
        Bank::AlphaBank => alphabank::extract_payment_data(&args.input_file),
        Bank::INGBank => ingbank::extract_payment_data(&args.input_file),
    };

    print_calculation_results(payment_data);

    press_btn_continue::wait("Apasati orice tasta pentru a inchide programul ...").unwrap();

    Ok(())
}

fn print_calculation_results(
    payment_data: (
        Vec<(NaiveDate, f64, f64, f64, f64, f64, f64)>,
        Vec<usize>,
        Vec<usize>,
    ),
) {
    let (payment_data, max_local_performance_indexes, max_global_performance_indexes) =
        payment_data;

    let index_column_width = max(5, payment_data.len().to_string().chars().count());
    let date_column_width = 10;
    let local_principal_column_width = max(
        "Capital".chars().count(),
        payment_data
            .iter()
            .map(|(_, local_principal, _, _, _, _, _)| {
                format!("{:.2} RON", local_principal).chars().count()
            })
            .max()
            .unwrap_or(0),
    );
    let local_interest_column_width = max(
        "Dobândă".chars().count(),
        payment_data
            .iter()
            .map(|(_, _, local_interest, _, _, _, _)| {
                format!("{:.2} RON", local_interest).chars().count()
            })
            .max()
            .unwrap_or(0),
    );
    let local_performance_column_width = max(
        "Raport".chars().count(),
        payment_data
            .iter()
            .map(|(_, _, _, local_performance, _, _, _)| {
                format!("{:.2}%", local_performance).chars().count()
            })
            .max()
            .unwrap_or(0),
    );
    let total_principal_column_width = max(
        "Capital total".chars().count(),
        payment_data
            .iter()
            .map(|(_, _, _, _, total_principal, _, _)| {
                format!("{:.2} RON", total_principal).chars().count()
            })
            .max()
            .unwrap_or(0),
    );
    let total_interest_column_width = max(
        "Dobândă totală".chars().count(),
        payment_data
            .iter()
            .map(|(_, _, _, _, _, total_interest, _)| {
                format!("{:.2} RON", total_interest).chars().count()
            })
            .max()
            .unwrap_or(0),
    );
    let total_performance_column_width = max(
        "Raport total".chars().count(),
        payment_data
            .iter()
            .map(|(_, _, _, _, _, _, total_performance)| {
                format!("{:.2}%", total_performance).chars().count()
            })
            .max()
            .unwrap_or(0),
    );
    let total_absolut_column_width = max(
        "Total absolut".chars().count(),
        payment_data
            .iter()
            .map(|(_, _, _, _, _, _, total_performance)| {
                format!("{:.2} RON", total_performance).chars().count()
            })
            .max()
            .unwrap_or(0),
    );

    payment_data
        .iter()
        .enumerate()
        .for_each(|(index, payment)| {
            if index % 12 == 0 {
                table_header(
                    index_column_width,
                    date_column_width,
                    local_principal_column_width,
                    local_interest_column_width,
                    local_performance_column_width,
                    total_principal_column_width,
                    total_interest_column_width,
                    total_performance_column_width,
                    total_absolut_column_width,
                );
            }

            let (
                date,
                local_principal,
                local_interest,
                local_performance,
                total_principal,
                total_interest,
                total_performance,
            ) = payment;
            let is_max_local_performance = max_local_performance_indexes.contains(&index);
            let is_max_global_performance = max_global_performance_indexes.contains(&index);

            let mut line_strings = Vec::new();
            line_strings.push(format!("{:width$}", index + 1, width = index_column_width));
            line_strings.push(format!("{:width$}", date, width = date_column_width));
            line_strings.push(format!(
                "{:>width$}",
                format!("{:.2} RON", local_principal),
                width = local_principal_column_width
            ));
            line_strings.push(format!(
                "{:>width$}",
                format!("{:.2} RON", local_interest),
                width = local_interest_column_width
            ));
            let local_performance_string = format!(
                "{:>width$}",
                format!("{:.2}%", local_performance),
                width = local_performance_column_width
            );
            line_strings.push(format!(
                "{:>width$}",
                match is_max_local_performance {
                    true => local_performance_string.blue().bold(),
                    false => {
                        let starting_index = index.saturating_sub(12);
                        let ending_index = index.saturating_sub(1);
                        let median_local_performance_over_last_12_months = payment_data
                            [starting_index..=ending_index]
                            .iter()
                            .map(|(_, _, _, local_performance, _, _, _)| local_performance)
                            .sum::<f64>()
                            / (ending_index - starting_index + 1) as f64;

                        if local_performance >= &median_local_performance_over_last_12_months {
                            local_performance_string.green()
                        } else {
                            local_performance_string.red()
                        }
                    }
                },
                width = local_performance_column_width
            ));
            line_strings.push(format!(
                "{:>width$}",
                format!("{:.2} RON", total_principal),
                width = total_principal_column_width
            ));
            line_strings.push(format!(
                "{:>width$}",
                format!("{:.2} RON", total_interest),
                width = total_interest_column_width
            ));
            let global_performance_string = format!(
                "{:>width$}",
                format!("{:.2}%", total_performance),
                width = total_performance_column_width
            );
            line_strings.push(format!(
                "{:>width$}",
                match is_max_global_performance {
                    true => global_performance_string.blue().bold(),
                    false => {
                        let starting_index = index.saturating_sub(12);
                        let ending_index = index.saturating_sub(1);
                        let median_global_performance_over_last_12_months = payment_data
                            [starting_index..=ending_index]
                            .iter()
                            .map(|(_, _, _, _, _, _, total_performance)| total_performance)
                            .sum::<f64>()
                            / (ending_index - starting_index + 1) as f64;

                        if total_performance >= &median_global_performance_over_last_12_months {
                            global_performance_string.green()
                        } else {
                            global_performance_string.red()
                        }
                    }
                },
                width = total_performance_column_width
            ));
            line_strings.push(format!(
                "{:>width$}",
                format!("{:.2} RON", total_principal + total_interest),
                width = total_absolut_column_width
            ));

            println!("| {} |", line_strings.join(" | "));
        });

    dashed_line(
        index_column_width,
        date_column_width,
        local_principal_column_width,
        local_interest_column_width,
        local_performance_column_width,
        total_principal_column_width,
        total_interest_column_width,
        total_performance_column_width,
        total_absolut_column_width,
    );
}
