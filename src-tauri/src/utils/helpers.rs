pub fn format_amount(amount: i64, decimals: u8) -> String {
    let denominator = 10u64.pow(decimals as u32) as f64;
    format!("{:.9}", amount as f64 / denominator)
}
