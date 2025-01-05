use finnit_abi::Transaction;

use crate::traits;

impl traits::TableRow for Transaction {
    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.account.clone(),
            self.datetime.to_string(),
            self.source.clone(),
            self.target.clone(),
            self.amount.to_string(),
            self.description.clone(),
        ]
    }
}
