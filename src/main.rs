use chrono::NaiveDate;
use colored::Colorize;
use std::cmp::max;
use std::error::Error;
use std::path::Path;
use walkdir::WalkDir;

mod alphabank;
mod ingbank;

fn main() -> Result<(), Box<dyn Error>> {
    let input_folder = Path::new("scadentare");
    if !input_folder.exists() || !input_folder.is_dir() {
        std::fs::create_dir_all(input_folder)?;
        println!("Am creat directorul: {:?}", input_folder);
    }

    let pdf_files = WalkDir::new(input_folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |e| e == "pdf"))
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    if pdf_files.is_empty() {
        println!("Nu am gasit fisiere PDF in directorul {:?}", input_folder);
        return Ok(());
    }

    let alpha_bank_files = pdf_files
        .iter()
        .enumerate()
        .filter_map(|(i, file)| file.file_name().and_then(|f| f.to_str().map(|f| (i, f))))
        .filter(|(_, f)| f.starts_with("alphabank"))
        .collect::<Vec<_>>();
    println!(
        "Am gasit {} fisiere AlphaBank in directorul {:?}",
        alpha_bank_files.len(),
        input_folder
    );
    alpha_bank_files.iter().for_each(|(i, f)| {
        println!("Procesare fisier: {}", f);
        let payment_data = alphabank::extract_payment_data(&pdf_files[*i]);
        print_calculation_results(payment_data);
    });

    let ing_bank_files = pdf_files
        .iter()
        .enumerate()
        .filter_map(|(i, file)| file.file_name().and_then(|f| f.to_str().map(|f| (i, f))))
        .filter(|(_, f)| f.starts_with("ingbank"))
        .collect::<Vec<_>>();
    println!(
        "Am gasit {} fisiere ING Bank in directorul {:?}",
        ing_bank_files.len(),
        input_folder
    );
    ing_bank_files.iter().for_each(|(i, f)| {
        println!("Procesare fisier: {}", f);
        let payment_data = ingbank::extract_payment_data(&pdf_files[*i]);
        print_calculation_results(payment_data);
    });

    press_btn_continue::wait("Apasati orice tasta pentru a inchide programul...").unwrap();

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

    let index_column_width = max(5, payment_data.len().to_string().len());
    let date_column_width = 10;
    let local_principal_column_width = max(
        "Capital".len(),
        payment_data
            .iter()
            .map(|(_, local_principal, _, _, _, _, _)| format!("{:.2} RON", local_principal).len())
            .max()
            .unwrap_or(0),
    );
    let local_interest_column_width = max(
        "Dobândă".len(),
        payment_data
            .iter()
            .map(|(_, _, local_interest, _, _, _, _)| format!("{:.2} RON", local_interest).len())
            .max()
            .unwrap_or(0),
    );
    let local_performance_column_width = max(
        "Raport".len(),
        payment_data
            .iter()
            .map(|(_, _, _, local_performance, _, _, _)| format!("{:.2}%", local_performance).len())
            .max()
            .unwrap_or(0),
    );
    let total_principal_column_width = max(
        "Capital total".len(),
        payment_data
            .iter()
            .map(|(_, _, _, _, total_principal, _, _)| format!("{:.2} RON", total_principal).len())
            .max()
            .unwrap_or(0),
    );
    let total_interest_column_width = max(
        "Dobândă totală".len(),
        payment_data
            .iter()
            .map(|(_, _, _, _, _, total_interest, _)| format!("{:.2} RON", total_interest).len())
            .max()
            .unwrap_or(0),
    );
    let total_performance_column_width = max(
        "Raport total".len(),
        payment_data
            .iter()
            .map(|(_, _, _, _, _, _, total_performance)| format!("{:.2}%", total_performance).len())
            .max()
            .unwrap_or(0),
    );
    let total_absolut_column_width = max(
        "Total absolut".len(),
        payment_data
            .iter()
            .map(|(_, _, _, _, _, _, total_performance)| {
                format!("{:.2} RON", total_performance).len()
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

                        if local_performance > &median_local_performance_over_last_12_months {
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

                        if total_performance > &median_global_performance_over_last_12_months {
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

            // println!("| {:5} | {:?} | {:12.2} RON | {:12.2} RON | {} | {:12.2} RON | {:12.2} RON | {} | {:12.2} RON |",
            //     index+1,
            //     date,
            //     local_principal,
            //     local_interest,{
            //         let local_performance_string= format!("{:11.2}%", local_performance);
            //     match is_max_local_performance {
            //         true => local_performance_string.blue().bold(),
            //         false => {
            //             let starting_index = index.saturating_sub(12);
            //             let ending_index = index.saturating_sub(1);
            //             let median_local_performance_over_last_12_months = payment_data[starting_index..=ending_index]
            //                 .iter()
            //                 .map(|(_, _, _, local_performance, _, _, _)| local_performance)
            //                 .sum::<f64>() / (ending_index - starting_index + 1) as f64;

            //             if local_performance > &median_local_performance_over_last_12_months {
            //                 local_performance_string.green()
            //             } else {
            //                 local_performance_string.red()
            //             }
            //         }
            //     }},
            //     total_principal,
            //     total_interest,{
            //         let global_performance_string= format!("{:11.2}%", total_performance);
            //         match is_max_global_performance {
            //             true => global_performance_string.blue().bold(),
            //             false => {
            //                 let starting_index = index.saturating_sub(12);
            //                 let ending_index = index.saturating_sub(1);
            //                 let median_global_performance_over_last_12_months = payment_data[starting_index..=ending_index]
            //                     .iter()
            //                     .map(|(_, _, _, _, _, _, total_performance)| total_performance)
            //                     .sum::<f64>() / (ending_index - starting_index + 1) as f64;

            //                 if total_performance > &median_global_performance_over_last_12_months {
            //                     global_performance_string.green()
            //                 } else {
            //                     global_performance_string.red()
            //                 }
            //             }
            //         }},
            //     total_principal + total_interest,
            // );
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

fn table_header(
    index_column_width: usize,
    date_column_width: usize,
    local_principal_column_width: usize,
    local_interest_column_width: usize,
    local_performance_column_width: usize,
    total_principal_column_width: usize,
    total_interest_column_width: usize,
    total_performance_column_width: usize,
    total_absolut_column_width: usize,
) {
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

    println!(
        "| {:index_column_width$} | {:date_column_width$} | {:local_principal_column_width$} | {:local_interest_column_width$} | {:local_performance_column_width$} | {:total_principal_column_width$} | {:total_interest_column_width$} | {:total_performance_column_width$} | {:total_absolut_column_width$} |",
        "Număr",
        "Dată",
        "Capital",
        "Dobândă",
        "Raport",
        "Capital total",
        "Dobândă totală",
        "Raport total",
        "Total absolut",
        index_column_width = index_column_width,
        date_column_width = date_column_width,
        local_principal_column_width = local_principal_column_width,
        local_interest_column_width = local_interest_column_width,
        local_performance_column_width = local_performance_column_width,
        total_principal_column_width = total_principal_column_width,
        total_interest_column_width = total_interest_column_width,
        total_performance_column_width = total_performance_column_width,
        total_absolut_column_width = total_absolut_column_width,
    );
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

fn dashed_line(
    index_column_width: usize,
    date_column_width: usize,
    local_principal_column_width: usize,
    local_interest_column_width: usize,
    local_performance_column_width: usize,
    total_principal_column_width: usize,
    total_interest_column_width: usize,
    total_performance_column_width: usize,
    total_absolut_column_width: usize,
) {
    println!(
        "+-{}-+-{}-+-{}-+-{}-+-{}-+-{}-+-{}-+-{}-+-{}-+",
        "-".repeat(index_column_width),
        "-".repeat(date_column_width),
        "-".repeat(local_principal_column_width),
        "-".repeat(local_interest_column_width),
        "-".repeat(local_performance_column_width),
        "-".repeat(total_principal_column_width),
        "-".repeat(total_interest_column_width),
        "-".repeat(total_performance_column_width),
        "-".repeat(total_absolut_column_width),
    );
}
