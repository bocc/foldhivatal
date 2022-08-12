use crate::contract::{Choice2, Contract, ContractMessage, ContractState};
use std::marker::PhantomData;

pub struct SellAHouse<State: ContractState, Message: ContractMessage> {
    pub state: State,
    pub message: Message,
}

pub struct Initiated {
    pub started_at: u32,
}

impl ContractState for Initiated {
    type Previous = ();
}

pub enum InitialMessageType<A, R> {
    Approve(PhantomData<A>),
    Reject(PhantomData<R>),
}
pub type InitialMessage = InitialMessageType<Approved, Rejected>;
impl ContractMessage for InitialMessage {}

impl Contract for SellAHouse<Initiated, InitialMessage> {
    type Message = InitialMessage;
    type Result<S: ContractState, M: ContractMessage> = SellAHouse<S, M>;

    fn process<S: ContractState, M: ContractMessage>(
        self,
        msg: Self::Message,
    ) -> Self::Result<S, M> {
        match msg {
            InitialMessageType::Approve(_) => todo!(),
            InitialMessageType::Reject(_) => todo!(),
        }
    }
}

pub enum EmptyMessage {}
impl ContractMessage for EmptyMessage {}

pub struct Rejected {}
impl ContractState for Rejected {
    type Previous = Initiated;
}

pub struct Approved {}
impl ContractState for Approved {
    type Previous = Initiated;
}
