use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{EgldOrMultiDctPayment, ManagedAddress, ManagedBuffer},
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall, ContractCallWithEgld};

/// Holds data for calling another contract, with any type of payment: none, MOA, Multi-DCT.
///
/// Gets created when chaining method `with_any_payment`.
#[must_use]
pub struct ContractCallWithAnyPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub basic: ContractCallNoPayment<SA, OriginalResult>,
    pub payment: EgldOrMultiDctPayment<SA>,
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithAnyPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithEgld<SA, Self::OriginalResult> {
        match self.payment {
            EgldOrMultiDctPayment::Moa(moa_amount) => self.basic.with_moa_transfer(moa_amount),
            EgldOrMultiDctPayment::MultiDct(multi_dct_payment) => self
                .basic
                .into_normalized()
                .convert_to_dct_transfer_call(multi_dct_payment),
        }
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        match self.payment {
            EgldOrMultiDctPayment::Moa(moa_amount) => {
                self.basic.transfer_execute_moa(moa_amount);
            },
            EgldOrMultiDctPayment::MultiDct(multi_dct_payment) => {
                self.basic.transfer_execute_dct(multi_dct_payment);
            },
        }
    }
}

impl<SA, OriginalResult> ContractCallWithAnyPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    /// Creates a new instance directly.
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        payment: EgldOrMultiDctPayment<SA>,
    ) -> Self {
        ContractCallWithAnyPayment {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            payment,
        }
    }
}
