use std::cmp::max;

use chrono::NaiveDate;

pub(crate) fn extract_payment_data(
    pdf_file: &std::path::PathBuf,
) -> (
    Vec<(NaiveDate, f64, f64, f64, f64, f64, f64)>,
    Vec<usize>,
    Vec<usize>,
    usize,
    usize,
) {
    let text = pdf_extract::extract_text(pdf_file).unwrap();
    let lines = text.lines().collect::<Vec<_>>();

    let date_line_indexes = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            if words.len() < 3 {
                return false;
            }
            NaiveDate::parse_from_str(
                &(format!("{} {} {}", words[0], words[1], words[2])),
                "%d %b %Y",
            )
            .is_ok()
        })
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let concatenated_lines = date_line_indexes
        .iter()
        .enumerate()
        .map(|(i, &date_line_index)| {
            let next_date_line_index = date_line_indexes.get(i + 1).cloned().unwrap_or(lines.len());
            lines[date_line_index..next_date_line_index].join(" ")
        })
        .filter(|line| line.split_whitespace().collect::<Vec<_>>().len() >= 19)
        .collect::<Vec<_>>();

    let mut total_principal = 0.0;
    let mut total_interest = 0.0;

    let mut max_local_performance = 0.0;
    let mut max_local_performance_indexes = Vec::new();
    let mut max_global_performance = 0.0;
    let mut max_global_performance_indexes = Vec::new();
    let payment_data: Vec<(NaiveDate, f64, f64, f64, f64, f64, f64)> = concatenated_lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            let date = format!("{} {} {}", words[0], words[1], words[2]);
            let date = NaiveDate::parse_from_str(&date, "%d %b %Y").unwrap();

            let local_principal = words[4].replace(",", "").parse().unwrap();
            let local_interest = words[6].replace(",", "").parse().unwrap();
            let local_performance = local_interest / local_principal * 100.0;
            if local_performance > max_local_performance {
                max_local_performance = local_performance;
                max_local_performance_indexes.clear();
                max_local_performance_indexes.push(i);
            } else if local_performance == max_local_performance {
                max_local_performance_indexes.push(i);
            }

            total_principal += local_principal;
            total_interest += local_interest;
            let total_performance = total_interest / total_principal * 100.0;
            if total_performance > max_global_performance {
                max_global_performance = total_performance;
                max_global_performance_indexes.clear();
                max_global_performance_indexes.push(i);
            } else if total_performance == max_global_performance {
                max_global_performance_indexes.push(i);
            }

            (
                date,
                local_principal,
                local_interest,
                local_performance,
                total_principal,
                total_interest,
                total_performance,
            )
        })
        .collect();

    let max_number_length = (total_principal + total_interest).to_string().len();
    let max_percentage_length = max(
        max_local_performance.to_string().len(),
        max_global_performance.to_string().len(),
    );

    (
        payment_data,
        max_local_performance_indexes,
        max_global_performance_indexes,
        max_number_length,
        max_percentage_length,
    )
}
