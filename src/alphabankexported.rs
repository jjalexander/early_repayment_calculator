use chrono::NaiveDate;

pub(crate) fn extract_payment_data(
    pdf_file: &std::path::PathBuf,
) -> (
    Vec<(NaiveDate, f64, f64, f64, f64, f64, f64)>,
    Vec<usize>,
    Vec<usize>,
) {
    let text = pdf_extract::extract_text(pdf_file).unwrap();
    let lines = text.lines().collect::<Vec<_>>();

    // remove the first 6 lines
    let lines = lines[6..].to_vec();

    // remove lines starting with "Nr"
    let lines = lines
        .iter()
        .filter(|line| !line.starts_with("Nr"))
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    // remove lines starting with "rata"
    let lines = lines
        .iter()
        .filter(|line| !line.starts_with("rata"))
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    // remove all empty lines
    let lines = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    // replace multiple spaces with a single space
    let lines = lines
        .iter()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>();

    // merge every 2 lines
    let lines = lines
        .chunks(2)
        .map(|chunk| chunk.join(" "))
        .collect::<Vec<_>>();

    // for (i, line) in lines.iter().enumerate() {
    //     println!("{}: {}", i, line);
    // }

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

            let date = format!("{} {} {}", words[0], words[1], words[2]);
            let date = NaiveDate::parse_from_str(words[1], "%d.%m.%Y").unwrap();

            let local_principal = words[4].parse().unwrap();
            let local_interest = words[5].parse().unwrap();
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
