// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            7
// Async Callback (empty):               1
// Total number of exported functions:   9

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    crowdfunding
    (
        init => init
        fund => fund
        status => status
        getCurrentFunds => get_current_funds
        claim => claim
        getTarget => target
        getDeadline => deadline
        getDeposit => deposit
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
