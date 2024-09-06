use chrono::NaiveDate;
use colored::Colorize;
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

    println!("Am gasit {} fisiere AlphaBank in directorul {:?}", alpha_bank_files.len(), input_folder);
            alpha_bank_files.iter().for_each(|(i, f)| {
                println!("Procesare fisier: {}", f);
                let payment_data = alphabank::extract_payment_data(&pdf_files[*i]);
                print_calculation_results(payment_data);
            });

    let ing_bank_files =


    pdf_files
        .iter()
        .enumerate()
        .filter_map(|(i, file)| file.file_name().and_then(|f| f.to_str().map(|f| (i, f))))
        .filter(|(_, f)| f.starts_with("ingbank"))
        .collect::<Vec<_>>();

    println!("Am gasit {} fisiere ING Bank in directorul {:?}", ing_bank_files.len(), input_folder);
    ing_bank_files.iter().for_each(|(i, f)| {
        println!("Procesare fisier: {}", f);
        let payment_data = ingbank::extract_payment_data(&pdf_files[*i]);
        print_calculation_results(payment_data);
    });

    press_btn_continue::wait("Apasati orice tasta pentru a inchide programul").unwrap();

    Ok(())
}

fn print_calculation_results(
    payment_data: (
        Vec<(NaiveDate, f64, f64, f64, f64, f64, f64)>,
        Vec<usize>,
        Vec<usize>,
        usize,
        usize,
    ),
) {
    let (
        payment_data,
        max_local_performance_indexes,
        max_global_performance_indexes,
        _max_number_length,
        _max_percentage_length,
    ) = payment_data;
    payment_data.iter().enumerate().for_each(|(index, payment)| {
            if index % 12 == 0 {
                table_header();
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

            println!("| {:5} | {:?} | {:11.2} RON | {:11.2} RON | {} | {:11.2} RON | {:11.2} RON | {} | {:11.2} RON |",
                index+1,
                date,
                local_principal,
                local_interest,
                match is_max_local_performance {
                    true => format!("{:10.2}%", local_performance).blue().bold(),
                    false => {
                        let starting_index = index.saturating_sub(12);
                        let ending_index = index.saturating_sub(1);
                        let median_local_performance_over_last_12_months = payment_data[starting_index..=ending_index]
                            .iter()
                            .map(|(_, _, _, local_performance, _, _, _)| local_performance)
                            .sum::<f64>() / (ending_index - starting_index + 1) as f64;

                        if local_performance > &median_local_performance_over_last_12_months {
                            format!("{:10.2}%", local_performance).green()
                        } else {
                            format!("{:10.2}%", local_performance).red()
                        }
                    }
                },
                total_principal,
                total_interest,
                match is_max_global_performance{
                    true => format!("{:10.2}%", total_performance).blue().bold(),
                    false => {
                        let previos_global_performance = payment_data.get(index.saturating_sub(1)).map(|(_, _, _, _, _, _, total_performance)| total_performance).unwrap_or(&0.0);

                        if total_performance > previos_global_performance {
                            format!("{:10.2}%", total_performance).green()
                        } else {
                            format!("{:10.2}%", total_performance).red()
                        }
                    }
                },
                total_principal + total_interest,
            );
        });
    dashed_line();
}

fn table_header() {
    dashed_line();
    println!(
        "| Index | Date       | Local Principal | Local Interest  | Local perf. | Total Principal | Total Interest  | Total perf. | Absolute Total  |"
    );
    dashed_line();
}

fn dashed_line() {
    println!(
        "+-------+------------+-----------------+-----------------+-------------+-----------------+-----------------+-------------+-----------------+"
    );
}
