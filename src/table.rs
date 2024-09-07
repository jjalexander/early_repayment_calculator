pub(crate) fn table_header(
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

pub(crate) fn dashed_line(
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
