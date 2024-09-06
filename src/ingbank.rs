use chrono::NaiveDate;

pub(crate) fn extract_payment_data(
    pdf_file: &std::path::PathBuf,
) -> (
    Vec<(NaiveDate, f64, f64, f64, f64, f64, f64)>,
    Vec<usize>,
    Vec<usize>,
) {
    let text = pdf_extract::extract_text(pdf_file).unwrap();
    let lines = text
        .lines()
        .filter(|line| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            if words.len() != 6 {
                return false;
            }
            NaiveDate::parse_from_str(&words[0], "%d.%m.%Y").is_ok()
        })
        .collect::<Vec<_>>();

    let mut total_principal = 0.0;
    let mut total_interest = 0.0;

    let mut max_local_performance = 0.0;
    let mut max_local_performance_indexes = Vec::new();
    let mut max_global_performance = 0.0;
    let mut max_global_performance_indexes = Vec::new();
    let payment_data: Vec<(NaiveDate, f64, f64, f64, f64, f64, f64)> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            let date = NaiveDate::parse_from_str(&words[0], "%d.%m.%Y").unwrap();

            let local_principal = words[3].parse().unwrap();
            let local_only_interest: f64 = words[2].parse().unwrap();
            let local_insurance: f64 = words[5].parse().unwrap();
            let local_interest = local_only_interest + local_insurance;
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

    (
        payment_data,
        max_local_performance_indexes,
        max_global_performance_indexes,
    )
}
