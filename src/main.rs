mod balances;
mod system;

fn main() {
    let mut balance = balances::Pallet::new();
    let mut system = system::Pallet::new();
}
