// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            3
// Async Callback (empty):               1
// Total number of exported functions:   5

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    second_contract
    (
        init => init
        acceptDctPayment => accept_dct_payment
        rejectDctPayment => reject_dct_payment
        getdctTokenName => get_contract_dct_token_identifier
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
