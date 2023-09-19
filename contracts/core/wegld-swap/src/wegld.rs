#![no_std]

dharitri_sc::imports!();

#[dharitri_sc::contract]
pub trait EgldDctSwap: dharitri_sc_modules::pause::PauseModule {
    #[init]
    fn init(&self, wrapped_moa_token_id: TokenIdentifier) {
        self.wrapped_moa_token_id().set(&wrapped_moa_token_id);
    }

    // endpoints

    #[payable("MOA")]
    #[endpoint(wrapEgld)]
    fn wrap_moa(&self) -> DctTokenPayment<Self::Api> {
        self.require_not_paused();

        let payment_amount = self.call_value().moa_value();
        require!(*payment_amount > 0u32, "Payment must be more than 0");

        let wrapped_moa_token_id = self.wrapped_moa_token_id().get();
        self.send()
            .dct_local_mint(&wrapped_moa_token_id, 0, &payment_amount);

        let caller = self.blockchain().get_caller();
        self.send()
            .direct_dct(&caller, &wrapped_moa_token_id, 0, &payment_amount);

        DctTokenPayment::new(wrapped_moa_token_id, 0, payment_amount.clone_value())
    }

    #[payable("*")]
    #[endpoint(unwrapEgld)]
    fn unwrap_moa(&self) {
        self.require_not_paused();

        let (payment_token, payment_amount) = self.call_value().single_fungible_dct();
        let wrapped_moa_token_id = self.wrapped_moa_token_id().get();

        require!(payment_token == wrapped_moa_token_id, "Wrong dct token");
        require!(payment_amount > 0u32, "Must pay more than 0 tokens!");
        require!(
            payment_amount <= self.get_locked_moa_balance(),
            "Contract does not have enough funds"
        );

        self.send()
            .dct_local_burn(&wrapped_moa_token_id, 0, &payment_amount);

        // 1 wrapped eGLD = 1 eGLD, so we pay back the same amount
        let caller = self.blockchain().get_caller();
        self.send().direct_moa(&caller, &payment_amount);
    }

    #[view(getLockedEgldBalance)]
    fn get_locked_moa_balance(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&EgldOrDctTokenIdentifier::moa(), 0)
    }

    #[view(getWrappedEgldTokenId)]
    #[storage_mapper("wrappedEgldTokenId")]
    fn wrapped_moa_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
