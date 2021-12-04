use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Method {
    MINT = 0
}

// #[derive(BorshSerialize, BorshDeserialize, Debug)]
// pub struct ProgramArgument {
//     pub value: u64
// }

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ProgramData {
    pub method: Method,
    pub args: u128
}


