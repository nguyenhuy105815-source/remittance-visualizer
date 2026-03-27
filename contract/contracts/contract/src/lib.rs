#![no_std]
use soroban_sdk::{contract, contractimpl, Env, symbol_short, Symbol};

#[contract]
pub struct RemittanceVisualizer;

#[contractimpl]
impl RemittanceVisualizer {
    /// Hàm mô phỏng gửi tiền để lấy dữ liệu so sánh phí
    /// amount: Số tiền giả định người dùng muốn chuyển (ví dụ: 100 USD)
    pub fn simulate_transfer(env: Env, amount: i128) -> i128 {
        // 1. Giả định phí SWIFT truyền thống là 5% + $25 cố định
        // Chúng ta tính toán ở đây để minh họa sự khác biệt
        let swift_fee = (amount * 5 / 100) + 25;

        // 2. Phát một Event lên mạng Stellar để chứng minh giao dịch đã thực hiện
        // Event này chứa: [Số tiền gửi, Phí truyền thống ước tính]
        env.events().publish(
            (symbol_short!("remit"), symbol_short!("data")),
            (amount, swift_fee)
        );

        // 3. Trả về 0 để xác nhận hàm chạy thành công
        // Phí thực tế của Stellar sẽ được mạng lưới tự động tính (thường là 0.00001 XLM)
        0
    }
}