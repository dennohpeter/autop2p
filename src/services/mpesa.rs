//! src/services/mpesa.rs
use dotenv::dotenv;
use mpesa::Mpesa;
use std::env;

pub struct MpesaService {}

impl MpesaService {
    pub fn new() -> Mpesa {
        dotenv().ok();
        Mpesa::new(
            env::var("MPESA_API_KEY").unwrap(),
            env::var("MPESA_SECRET_KEY").unwrap(),
            Environment::Sandbox,
        )
    }

    pub fn get_balance(&self) -> Result<Balance, Error> {
        self.get_balance()
    }

    pub fn get_transaction_status(&self, transaction_id: &str) -> Result<TransactionStatus, Error> {
        self.get_transaction_status(transaction_id)
    }

    pub fn get_transaction_status_by_receipt(
        &self,
        transaction_id: &str,
        receipt_number: &str,
    ) -> Result<TransactionStatus, Error> {
        self.get_transaction_status_by_receipt(transaction_id, receipt_number)
    }
}
