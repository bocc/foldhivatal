pub trait Contract {
    type Message;
    type Result<S: ContractState, M: ContractMessage>;

    fn process<S: ContractState, M: ContractMessage>(
        self,
        msg: Self::Message,
    ) -> Self::Result<S, M>;
}

pub trait Initiate<C: Contract> {
    fn initiate() -> C;
}

pub trait ContractMessage {}

pub trait ContractState {
    type Previous;
}
pub enum Choice2<A, B> {
    First(A),
    Second(B),
}
