#[macro_export]
macro_rules! String {
    ($x: expr) => {
        String::from($x)
    };
}

#[macro_export]
macro_rules! create_block {
    ($block_number:expr, $(($caller:expr, $to:expr, $amount:expr)),* $(,)?) => {
        support::Block {
            header: support::Header { block_number: $block_number },
            extrinsics: vec![
                $(
                    support::Extrinsic {
                        caller: $caller.clone(),
                        call: RuntimeCall::Balances(balances::Call::Transfer {
                            to: $to.clone(),
                            amount: $amount,
                        }),
                    }
                ),*
            ],
        }
    };
}
