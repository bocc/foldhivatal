#![allow(unused)]
#![feature(generic_associated_types)]
// #![feature()]

// what do we want
// we want to have an inventory of real estate
// it maintains
//  - a set of attributes
//  - a set stakeholders (owners, lenders, owner representatives)
//  - a set of contracts the property is involved in that pose limitations to the eligibility of future contracts
//  - a log of events
//
// events can
//  - change any data and state associated with properties
//  - trigger notifications/requests
//
// contracts
//  - are a set of steps
//  - they can affect attributes, stakeholders and other contracts
//  - example: house, one owner, one buyer, bank
//      * buyer initiates contact
//      * buyer transfers deposit
//      * bank confirms deposit transfer
//      * no further ownership-impacting contracts can be created until deposit timeout (sell, loan) or double deposit payback
//      * bank confirms full amount transferred, sets itself as stakeholder, prohibiting ownership impacting contracts,

// what do we want, 2nd iter

// - a (n atomic) transaction log
// - we want to have queryable 'snapshots'
// - various entities with attributes
//      eg: locks, references to other entities
// - actors are not part of the log, they are validated externally, part of an append
//      only set

mod contract;
mod event;
mod inventory;
mod preperty;
mod sell_a_house;
mod stakeholder;
fn main() {}
