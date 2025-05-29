#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod refund_log {
    use apresh_store::get_memory;
    use candid::Principal;
    use ic_stable_structures::{
        memory_manager::{MemoryId, VirtualMemory},
        DefaultMemoryImpl, Log,
    };
    use crate::transfer::Refund;
    pub const REFUND_LOG_INDEX_MEMORY_ID: MemoryId = MemoryId::new(4);
    pub const REFUND_LOG_DATA_MEMORY_ID: MemoryId = MemoryId::new(5);
    pub struct RefundLog(
        Log<Refund, VirtualMemory<DefaultMemoryImpl>, VirtualMemory<DefaultMemoryImpl>>,
    );
    impl Default for RefundLog {
        fn default() -> Self {
            let index_mem = get_memory(REFUND_LOG_INDEX_MEMORY_ID);
            let data_mem = get_memory(REFUND_LOG_DATA_MEMORY_ID);
            Self(Log::init(index_mem, data_mem).unwrap())
        }
    }
    impl RefundLog {
        pub fn append(
            &self,
            amount: u64,
            recipient: Principal,
            memo: String,
        ) -> Result<u64, String> {
            let refund = Refund {
                amount,
                recipient,
                memo,
                timestamp: ic_cdk::api::time(),
            };
            self.0
                .append(&refund)
                .map_err(|err| ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!("Failed to log refund: {0:?}", err),
                    );
                    res
                }))
        }
        pub fn get(&self, idx: u64) -> Option<Refund> {
            self.0.get(idx)
        }
        pub fn iter(&self) -> impl Iterator<Item = Refund> + '_ {
            self.0.iter()
        }
    }
}
mod transfer {
    pub mod consts {
        pub const LEDGER_CANISTER_ID: &str = if true {
            "ryjl3-tyaaa-aaaaa-aaaba-cai"
        } else {
            "mxzaz-hqaaa-aaaar-qaada-cai"
        };
        pub const THIS_CANISTER_ID: &str = "vujqm-syaaa-aaaag-at46q-cai";
    }
    mod refund {
        use candid::{CandidType, Decode, Encode, Principal};
        use ic_stable_structures::{storable::Bound, Storable};
        use serde::Deserialize;
        use std::borrow::Cow;
        pub struct Refund {
            pub amount: u64,
            pub recipient: Principal,
            pub memo: String,
            pub timestamp: u64,
        }
        impl ::candid::types::CandidType for Refund {
            fn _ty() -> ::candid::types::Type {
                ::candid::types::TypeInner::Record(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                ::candid::types::Field {
                                    id: ::candid::types::Label::Named("memo".to_string())
                                        .into(),
                                    ty: <String as ::candid::types::CandidType>::ty(),
                                },
                                ::candid::types::Field {
                                    id: ::candid::types::Label::Named("recipient".to_string())
                                        .into(),
                                    ty: <Principal as ::candid::types::CandidType>::ty(),
                                },
                                ::candid::types::Field {
                                    id: ::candid::types::Label::Named("timestamp".to_string())
                                        .into(),
                                    ty: <u64 as ::candid::types::CandidType>::ty(),
                                },
                                ::candid::types::Field {
                                    id: ::candid::types::Label::Named("amount".to_string())
                                        .into(),
                                    ty: <u64 as ::candid::types::CandidType>::ty(),
                                },
                            ]),
                        ),
                    )
                    .into()
            }
            fn id() -> ::candid::types::TypeId {
                ::candid::types::TypeId::of::<Refund>()
            }
            fn idl_serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::std::result::Result<(), __S::Error>
            where
                __S: ::candid::types::Serializer,
            {
                let mut ser = __serializer.serialize_struct()?;
                ::candid::types::Compound::serialize_element(&mut ser, &self.memo)?;
                ::candid::types::Compound::serialize_element(&mut ser, &self.recipient)?;
                ::candid::types::Compound::serialize_element(&mut ser, &self.timestamp)?;
                ::candid::types::Compound::serialize_element(&mut ser, &self.amount)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Refund {
            #[inline]
            fn clone(&self) -> Refund {
                Refund {
                    amount: ::core::clone::Clone::clone(&self.amount),
                    recipient: ::core::clone::Clone::clone(&self.recipient),
                    memo: ::core::clone::Clone::clone(&self.memo),
                    timestamp: ::core::clone::Clone::clone(&self.timestamp),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Refund {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Refund",
                    "amount",
                    &self.amount,
                    "recipient",
                    &self.recipient,
                    "memo",
                    &self.memo,
                    "timestamp",
                    &&self.timestamp,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Refund {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "amount" => _serde::__private::Ok(__Field::__field0),
                                "recipient" => _serde::__private::Ok(__Field::__field1),
                                "memo" => _serde::__private::Ok(__Field::__field2),
                                "timestamp" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"amount" => _serde::__private::Ok(__Field::__field0),
                                b"recipient" => _serde::__private::Ok(__Field::__field1),
                                b"memo" => _serde::__private::Ok(__Field::__field2),
                                b"timestamp" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Refund>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Refund;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Refund",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Refund with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                Principal,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Refund with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Refund with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                u64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Refund with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Refund {
                                amount: __field0,
                                recipient: __field1,
                                memo: __field2,
                                timestamp: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u64> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Principal> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<u64> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "recipient",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Principal>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("memo"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "timestamp",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("amount")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("recipient")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("memo")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("timestamp")?
                                }
                            };
                            _serde::__private::Ok(Refund {
                                amount: __field0,
                                recipient: __field1,
                                memo: __field2,
                                timestamp: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "amount",
                        "recipient",
                        "memo",
                        "timestamp",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Refund",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Refund>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl Storable for Refund {
            const BOUND: Bound = Bound::Unbounded;
            fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
                Cow::Owned(
                    {
                        let mut builder = ::candid::ser::IDLBuilder::new();
                        {
                            builder
                                .arg(self)
                                .and_then(|builder| { builder.serialize_to_vec() })
                        }
                    }
                        .unwrap(),
                )
            }
            fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
                {
                    {
                        ::candid::de::IDLDeserialize::new_with_config(
                                bytes.as_ref(),
                                &::candid::de::DecoderConfig::new(),
                            )
                            .and_then(|mut de| {
                                { de.get_value::<Self>().and_then(|val| { Ok((val)) }) }
                                    .and_then(|res| de.done().and(Ok(res)))
                            })
                    }
                }
                    .unwrap()
            }
        }
    }
    mod transfer_in {
        use crate::transfer::utils::{
            get_account_from_principal, get_canister_default_account,
            get_ledger_principal,
        };
        use anyhow::anyhow;
        use icrc_ledger_types::{
            icrc1::transfer::BlockIndex,
            icrc2::transfer_from::{TransferFromArgs, TransferFromError},
        };
        use super::TransferInParams;
        pub async fn transfer_in(args: TransferInParams) -> anyhow::Result<()> {
            {
                ::std::io::_print(
                    format_args!(
                        "Transferring {0} tokens from account {1}\n",
                        &args.params.amount,
                        &args.from,
                    ),
                );
            };
            let transfer_from_args = TransferFromArgs {
                from: get_account_from_principal(ic_cdk::caller()),
                memo: args.params.memo,
                amount: args.params.amount,
                spender_subaccount: None,
                fee: None,
                to: get_canister_default_account(),
                created_at_time: None,
            };
            let block_index = ic_cdk::call::<
                (TransferFromArgs,),
                (Result<BlockIndex, TransferFromError>,),
            >(get_ledger_principal(), "icrc2_transfer_from", (transfer_from_args,))
                .await
                .map_err(|e| ::anyhow::Error::msg(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("failed to call ledger: {0:?}", e),
                        );
                        res
                    }),
                ))?
                .0
                .map_err(|e| ::anyhow::Error::msg(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("ledger transfer error {0:?}", e),
                        );
                        res
                    }),
                ))?;
            {
                ::std::io::_print(
                    format_args!(
                        "Transfer successful. Block index: {0:?}\n",
                        block_index,
                    ),
                );
            };
            Ok(())
        }
    }
    mod transfer_out {
        use candid::Nat;
        use icrc_ledger_types::icrc1::transfer::{BlockIndex, TransferArg, TransferError};
        use super::TransferOutParams;
        use crate::transfer::utils::get_ledger_principal;
        use anyhow::anyhow;
        pub async fn transfer_out(
            mut args: TransferOutParams,
            fee: u64,
        ) -> anyhow::Result<()> {
            args.params.amount -= Nat::from(fee);
            {
                ::std::io::_print(
                    format_args!(
                        "Transferring {0} tokens to account {1}\n",
                        &args.params.amount,
                        &args.to,
                    ),
                );
            };
            let transfer_args = TransferArg {
                from_subaccount: None,
                memo: args.params.memo,
                amount: args.params.amount,
                to: args.to,
                fee: Some(Nat::from(fee)),
                created_at_time: None,
            };
            let block_index = ic_cdk::call::<
                (TransferArg,),
                (Result<BlockIndex, TransferError>,),
            >(get_ledger_principal(), "icrc1_transfer", (transfer_args,))
                .await
                .map_err(|e| ::anyhow::Error::msg(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("failed to call ledger: {0:?}", e),
                        );
                        res
                    }),
                ))?
                .0
                .map_err(|e| ::anyhow::Error::msg(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("ledger transfer error {0:?}", e),
                        );
                        res
                    }),
                ))?;
            {
                ::std::io::_print(
                    format_args!(
                        "Transfer successful. Block index: {0:?}\n",
                        block_index,
                    ),
                );
            };
            Ok(())
        }
    }
    mod utils {
        use candid::Principal;
        use icrc_ledger_types::icrc1::account::Account;
        use super::consts::LEDGER_CANISTER_ID;
        pub fn get_account_from_principal(principal: Principal) -> Account {
            Account::from(principal)
        }
        pub fn get_canister_default_account() -> Account {
            get_account_from_principal(ic_cdk::api::id())
        }
        pub fn get_ledger_principal() -> Principal {
            Principal::from_text(LEDGER_CANISTER_ID)
                .expect("Could not decode the principal.")
        }
    }
    use icrc_ledger_types::icrc1::{account::Account, transfer::{Memo, NumTokens}};
    pub use refund::Refund;
    pub use transfer_in::transfer_in;
    pub use transfer_out::transfer_out;
    pub struct TransferInParams {
        pub from: Account,
        pub params: TransferParams,
    }
    pub struct TransferOutParams {
        pub to: Account,
        pub params: TransferParams,
    }
    pub struct TransferParams {
        pub amount: NumTokens,
        pub memo: Option<Memo>,
    }
}
mod utils {
    use apresh_store::Guard;
    use balances::{balances, Balances};
    use candid::Principal;
    use icrc_ledger_types::icrc1::transfer::Memo;
    use crate::{ADMIN, CANISTER_LOCKED};
    fn is_admin() -> bool {
        ADMIN.with_borrow(|caller| *caller == ic_cdk::caller())
    }
    pub fn assert_admin() {
        if !is_admin() {
            ic_cdk::trap("Not authorized");
        }
    }
    pub fn assert_whitelisted() {
        if !crate::WHITELIST
            .with_borrow(|whitelist| whitelist.contains(&ic_cdk::caller()))
        {
            ic_cdk::trap("Not whitelisted");
        }
        if !is_admin() && CANISTER_LOCKED.with_borrow(|locked| *locked) {
            ic_cdk::trap("Canister is locked");
        }
    }
    pub fn memo(purpose: &str, amount: u64) -> Option<Memo> {
        if purpose.len() > 10 {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!("Memo purpose is longer than expected"),
                    ),
                );
            };
        }
        let memo = ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(
                    format_args!("Apresh: {0} of {1}", purpose, amount),
                );
                res
            })
            .as_bytes()
            .to_vec();
        if memo.len() > 32 {
            return None;
        }
        Some(Memo::from(memo))
    }
    pub fn callers_balances() -> Guard<Balances> {
        balances_of(ic_cdk::caller())
    }
    pub fn balances_of(principal: Principal) -> Guard<Balances> {
        let caller_bytes = principal.as_slice().to_vec();
        balances(caller_bytes)
    }
}
use std::cell::RefCell;
use apresh_engine::{
    operations::{
        AddMessageOp, BuyShipmentOp, CancelShipmentOp, CreateShipmentOp,
        FinalizeShipmentOp, ReadMessageOp, RegisterActorOp, StateOp,
    },
    state::CanisterState,
};
use apresh_qr_code::{generate, QrCodeOptions};
use apresh_store::Record;
use apresh_types::{
    ActorId, Carrier, CarrierKey, Channel, ChannelKey, PrintableShipment, Shipment,
    ShipmentInfo, ShipmentStatus, ShipperKey,
};
use candid::Principal;
use entrypoint::entrypoint;
use ic_cdk::{init, query, update};
use icrc_ledger_types::icrc1::transfer::NumTokens;
use refund_log::RefundLog;
pub use transfer::consts;
use transfer::{
    transfer_in, transfer_out, TransferInParams, TransferOutParams, TransferParams,
};
use utils::{assert_admin, assert_whitelisted, callers_balances, memo};
type ContractResult<T> = Result<T, String>;
pub const STATE: ::std::thread::LocalKey<RefCell<CanisterState>> = {
    #[inline]
    fn __init() -> RefCell<CanisterState> {
        RefCell::new(CanisterState::default())
    }
    unsafe {
        ::std::thread::LocalKey::new(const {
            if ::std::mem::needs_drop::<RefCell<CanisterState>>() {
                |init| {
                    #[thread_local]
                    static VAL: ::std::thread::local_impl::LazyStorage<
                        RefCell<CanisterState>,
                        (),
                    > = ::std::thread::local_impl::LazyStorage::new();
                    VAL.get_or_init(init, __init)
                }
            } else {
                |init| {
                    #[thread_local]
                    static VAL: ::std::thread::local_impl::LazyStorage<
                        RefCell<CanisterState>,
                        !,
                    > = ::std::thread::local_impl::LazyStorage::new();
                    VAL.get_or_init(init, __init)
                }
            }
        })
    }
};
pub const TRANSFER_FEE: ::std::thread::LocalKey<RefCell<u64>> = {
    const __INIT: RefCell<u64> = { RefCell::new(10_000) };
    unsafe {
        ::std::thread::LocalKey::new(const {
            if ::std::mem::needs_drop::<RefCell<u64>>() {
                |_| {
                    #[thread_local]
                    static VAL: ::std::thread::local_impl::EagerStorage<RefCell<u64>> = ::std::thread::local_impl::EagerStorage::new(
                        __INIT,
                    );
                    VAL.get()
                }
            } else {
                |_| {
                    #[thread_local]
                    static VAL: RefCell<u64> = __INIT;
                    &VAL
                }
            }
        })
    }
};
pub const ADMIN: ::std::thread::LocalKey<RefCell<Principal>> = {
    const __INIT: RefCell<Principal> = { RefCell::new(Principal::anonymous()) };
    unsafe {
        ::std::thread::LocalKey::new(const {
            if ::std::mem::needs_drop::<RefCell<Principal>>() {
                |_| {
                    #[thread_local]
                    static VAL: ::std::thread::local_impl::EagerStorage<
                        RefCell<Principal>,
                    > = ::std::thread::local_impl::EagerStorage::new(__INIT);
                    VAL.get()
                }
            } else {
                |_| {
                    #[thread_local]
                    static VAL: RefCell<Principal> = __INIT;
                    &VAL
                }
            }
        })
    }
};
pub const WHITELIST: ::std::thread::LocalKey<RefCell<Vec<Principal>>> = {
    #[inline]
    fn __init() -> RefCell<Vec<Principal>> {
        RefCell::default()
    }
    unsafe {
        ::std::thread::LocalKey::new(const {
            if ::std::mem::needs_drop::<RefCell<Vec<Principal>>>() {
                |init| {
                    #[thread_local]
                    static VAL: ::std::thread::local_impl::LazyStorage<
                        RefCell<Vec<Principal>>,
                        (),
                    > = ::std::thread::local_impl::LazyStorage::new();
                    VAL.get_or_init(init, __init)
                }
            } else {
                |init| {
                    #[thread_local]
                    static VAL: ::std::thread::local_impl::LazyStorage<
                        RefCell<Vec<Principal>>,
                        !,
                    > = ::std::thread::local_impl::LazyStorage::new();
                    VAL.get_or_init(init, __init)
                }
            }
        })
    }
};
pub const REFUND_LOG: ::std::thread::LocalKey<RefCell<RefundLog>> = {
    #[inline]
    fn __init() -> RefCell<RefundLog> {
        RefCell::new(RefundLog::default())
    }
    unsafe {
        ::std::thread::LocalKey::new(const {
            if ::std::mem::needs_drop::<RefCell<RefundLog>>() {
                |init| {
                    #[thread_local]
                    static VAL: ::std::thread::local_impl::LazyStorage<
                        RefCell<RefundLog>,
                        (),
                    > = ::std::thread::local_impl::LazyStorage::new();
                    VAL.get_or_init(init, __init)
                }
            } else {
                |init| {
                    #[thread_local]
                    static VAL: ::std::thread::local_impl::LazyStorage<
                        RefCell<RefundLog>,
                        !,
                    > = ::std::thread::local_impl::LazyStorage::new();
                    VAL.get_or_init(init, __init)
                }
            }
        })
    }
};
pub const CANISTER_LOCKED: ::std::thread::LocalKey<RefCell<bool>> = {
    const __INIT: RefCell<bool> = { RefCell::new(false) };
    unsafe {
        ::std::thread::LocalKey::new(const {
            if ::std::mem::needs_drop::<RefCell<bool>>() {
                |_| {
                    #[thread_local]
                    static VAL: ::std::thread::local_impl::EagerStorage<RefCell<bool>> = ::std::thread::local_impl::EagerStorage::new(
                        __INIT,
                    );
                    VAL.get()
                }
            } else {
                |_| {
                    #[thread_local]
                    static VAL: RefCell<bool> = __INIT;
                    &VAL
                }
            }
        })
    }
};
#[export_name = "canister_init"]
fn __canister_method_init() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let result = init();
    });
}
fn init() {
    ADMIN.with_borrow_mut(|caller| *caller = ic_cdk::caller());
}
#[export_name = "canister_query.balance"]
fn __canister_method_balance() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let () = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = balance();
        ic_cdk::api::call::reply(result)
    });
}
fn balance() -> (u64, u64) {
    let balances = callers_balances();
    (balances.balance(), balances.locked())
}
#[export_name = "canister_update.deposit"]
fn __canister_method_deposit() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let (amount,) = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = deposit(amount).await;
        ic_cdk::api::call::reply((result,))
    });
}
async fn deposit(amount: u64) -> ContractResult<()> {
    assert_whitelisted();
    if let Err(e) = transfer_in(TransferInParams {
            params: TransferParams {
                amount: NumTokens::from(amount),
                memo: memo("DEPOSIT", amount),
            },
            from: ic_cdk::caller().into(),
        })
        .await
    {
        ic_cdk::trap(&e.to_string());
    }
    let mut balances = callers_balances();
    balances.deposit(amount).map_err(|e| e.to_string())?;
    balances.commit();
    Ok(())
}
#[export_name = "canister_update.withdraw"]
fn __canister_method_withdraw() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let (amount,) = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = withdraw(amount).await;
        ic_cdk::api::call::reply((result,))
    });
}
async fn withdraw(amount: u64) -> ContractResult<()> {
    assert_whitelisted();
    let fee = get_transfer_fee();
    if amount <= fee {
        ic_cdk::trap("Insufficient balance");
    }
    let mut balances = callers_balances();
    balances.withdraw(amount).map_err(|e| e.to_string())?;
    let transfer_args = TransferOutParams {
        params: TransferParams {
            amount: NumTokens::from(amount),
            memo: memo("WITHDRAW", amount),
        },
        to: ic_cdk::caller().into(),
    };
    if let Err(e) = transfer_out(transfer_args, get_transfer_fee()).await {
        if let Err(e_log) = REFUND_LOG
            .with_borrow_mut(|log| {
                log.append(
                    amount,
                    ic_cdk::caller(),
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("ERROR WITHDRAW: {0}", e),
                        );
                        res
                    }),
                )
            })
        {
            ic_cdk::trap(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "Error while withdrawing and appending to log {0} {1}",
                            e,
                            e_log,
                        ),
                    );
                    res
                }),
            );
        }
        ic_cdk::trap(
            &::alloc::__export::must_use({
                let res = ::alloc::fmt::format(
                    format_args!("Error while withdrawing, {0}", e),
                );
                res
            }),
        );
    }
    balances.commit();
    Ok(())
}
#[export_name = "canister_query.is_mainnet"]
fn __canister_method_is_mainnet() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let () = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = is_mainnet();
        ic_cdk::api::call::reply((result,))
    });
}
fn is_mainnet() -> bool {
    true
}
#[export_name = "canister_update.addWhitelisted"]
fn __canister_method_add_whitelisted() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let (principal,) = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = add_whitelisted(principal);
        ic_cdk::api::call::reply(())
    });
}
fn add_whitelisted(principal: Principal) {
    assert_admin();
    WHITELIST.with_borrow_mut(|whitelist| whitelist.push(principal));
}
#[export_name = "canister_update.setTransferFee"]
fn __canister_method_set_transfer_fee() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let (fee,) = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = set_transfer_fee(fee);
        ic_cdk::api::call::reply(())
    });
}
fn set_transfer_fee(fee: u64) {
    assert_admin();
    TRANSFER_FEE.set(fee);
}
#[export_name = "canister_query.getTransferFee"]
fn __canister_method_get_transfer_fee() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let () = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = get_transfer_fee();
        ic_cdk::api::call::reply((result,))
    });
}
fn get_transfer_fee() -> u64 {
    TRANSFER_FEE.with_borrow(|fee| *fee)
}
#[export_name = "canister_update.add_message"]
fn __canister_method_add_message() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let (message, shipment) = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = add_message(message, shipment);
        ic_cdk::api::call::reply((result,))
    });
}
fn add_message(
    message: Vec<u8>,
    shipment: <Shipment as apresh_store::DatabaseKeyable>::Key,
) -> ContractResult<()> {
    fn inner_add_message(
        message: Vec<u8>,
        shipment: &mut Shipment,
    ) -> ContractResult<()> {
        {
            assert_whitelisted();
            let caller = ActorId(ic_cdk::caller());
            STATE
                .with_borrow_mut(|state| {
                    AddMessageOp::new(shipment, message, caller).apply(state)
                })
                .map_err(|e| e.to_string())
        }
    }
    let mut shipment = shipment.get().unwrap();
    let r = inner_add_message(message, &mut shipment);
    match &r {
        Ok(_) => {
            shipment.commit();
        }
        Err(_) => {
            shipment.revert();
        }
    }
    r
}
pub fn encode_add_message_args(
    message: Vec<u8>,
    shipment: <Shipment as apresh_store::DatabaseKeyable>::Key,
) -> Vec<u8> {
    use candid::Encode;
    {
        let mut builder = ::candid::ser::IDLBuilder::new();
        {
            builder
                .arg(&&message)
                .and_then(|builder| {
                    builder
                        .arg(&shipment)
                        .and_then(|builder| { builder.serialize_to_vec() })
                })
        }
    }
        .unwrap()
}
#[export_name = "canister_query.read_channel"]
fn __canister_method_read_channel() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let (shipment,) = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = read_channel(shipment);
        ic_cdk::api::call::reply((result,))
    });
}
fn read_channel(
    shipment: <Shipment as apresh_store::DatabaseKeyable>::Key,
) -> ContractResult<Channel> {
    fn inner_read_channel(shipment: &mut Shipment) -> ContractResult<Channel> {
        {
            let caller = ActorId(ic_cdk::caller());
            STATE
                .with_borrow(|state| ReadMessageOp::new(&shipment, caller).read(state))
                .map_err(|e| e.to_string())
        }
    }
    let mut shipment = shipment.get().unwrap();
    let r = inner_read_channel(&mut shipment);
    match &r {
        Ok(_) => {
            shipment.commit();
        }
        Err(_) => {
            shipment.revert();
        }
    }
    r
}
pub fn encode_read_channel_args(
    shipment: <Shipment as apresh_store::DatabaseKeyable>::Key,
) -> Vec<u8> {
    use candid::Encode;
    {
        let mut builder = ::candid::ser::IDLBuilder::new();
        { builder.arg(&&shipment).and_then(|builder| { builder.serialize_to_vec() }) }
    }
        .unwrap()
}
#[export_name = "canister_update.finalizeShipment"]
fn __canister_method_finalize_shipment() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let (shipment, secret_key) = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = finalize_shipment(shipment, secret_key);
        ic_cdk::api::call::reply((result,))
    });
}
fn finalize_shipment(
    shipment: <Shipment as apresh_store::DatabaseKeyable>::Key,
    secret_key: Option<String>,
) -> ContractResult<()> {
    fn inner_finalize_shipment(
        shipment: &mut Shipment,
        secret_key: Option<String>,
    ) -> ContractResult<()> {
        {
            let caller = ActorId(ic_cdk::caller());
            let result = STATE
                .with_borrow_mut(|state| {
                    FinalizeShipmentOp::new(shipment, secret_key, caller).apply(state)
                })
                .map_err(|e| e.to_string())?;
            let (mut shipper_balances, mut carrier_balances) = shipment
                .both_balances()
                .map_err(|e| e.to_string())?;
            let transfer_result = carrier_balances
                .transfer_from_and_unlock(
                    &mut shipper_balances,
                    result.price(),
                    result.value(),
                );
            match &transfer_result {
                Ok(_) => {
                    carrier_balances.commit();
                    shipper_balances.commit();
                }
                Err(_e) => {
                    carrier_balances.revert();
                    shipper_balances.revert();
                }
            };
            transfer_result.map_err(|e| e.to_string())
        }
    }
    let mut shipment = shipment.get().unwrap();
    let r = inner_finalize_shipment(&mut shipment, secret_key);
    match &r {
        Ok(_) => {
            shipment.commit();
        }
        Err(_) => {
            shipment.revert();
        }
    }
    r
}
pub fn encode_finalize_shipment_args(
    shipment: <Shipment as apresh_store::DatabaseKeyable>::Key,
    secret_key: Option<String>,
) -> Vec<u8> {
    use candid::Encode;
    {
        let mut builder = ::candid::ser::IDLBuilder::new();
        {
            builder
                .arg(&&shipment)
                .and_then(|builder| {
                    builder
                        .arg(&secret_key)
                        .and_then(|builder| { builder.serialize_to_vec() })
                })
        }
    }
        .unwrap()
}
#[export_name = "canister_update.buyShipment"]
fn __canister_method_buy_shipment() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let (carrier_name, shipment, channel_key) = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = buy_shipment(carrier_name, shipment, channel_key);
        ic_cdk::api::call::reply((result,))
    });
}
fn buy_shipment(
    carrier_name: Option<String>,
    shipment: <Shipment as apresh_store::DatabaseKeyable>::Key,
    channel_key: ChannelKey,
) -> ContractResult<()> {
    fn inner_buy_shipment(
        carrier_name: Option<String>,
        shipment: &mut Shipment,
        channel_key: ChannelKey,
    ) -> ContractResult<()> {
        {
            assert_whitelisted();
            let caller = ActorId(ic_cdk::caller());
            let mut carrier = CarrierKey(caller).get().unwrap();
            let result = STATE
                .with_borrow_mut(|state| {
                    if let Some(carrier_name) = carrier_name {
                        let carrier = Carrier::new(caller, carrier_name.as_str());
                        RegisterActorOp::AddCarrier {
                            id: carrier.id(),
                            name: carrier_name,
                        }
                            .apply(state)
                            .map_err(|e| e.to_string())
                            .unwrap();
                    }
                    BuyShipmentOp::new(&mut carrier, shipment, channel_key).apply(state)
                });
            let shipment_value = match result {
                Ok(shipment_value) => shipment_value,
                Err(e) => {
                    carrier.revert();
                    return Err(e.to_string());
                }
            };
            let mut balances = callers_balances();
            match balances.lock(shipment_value) {
                Ok(_) => {
                    carrier.commit();
                    balances.commit();
                    Ok(())
                }
                Err(e) => {
                    carrier.revert();
                    balances.revert();
                    Err(e.to_string())
                }
            }
        }
    }
    let mut shipment = shipment.get().unwrap();
    let r = inner_buy_shipment(carrier_name, &mut shipment, channel_key);
    match &r {
        Ok(_) => {
            shipment.commit();
        }
        Err(_) => {
            shipment.revert();
        }
    }
    r
}
pub fn encode_buy_shipment_args(
    carrier_name: Option<String>,
    shipment: <Shipment as apresh_store::DatabaseKeyable>::Key,
    channel_key: ChannelKey,
) -> Vec<u8> {
    use candid::Encode;
    {
        let mut builder = ::candid::ser::IDLBuilder::new();
        {
            builder
                .arg(&&carrier_name)
                .and_then(|builder| {
                    builder
                        .arg(&shipment)
                        .and_then(|builder| {
                            builder
                                .arg(&channel_key)
                                .and_then(|builder| { builder.serialize_to_vec() })
                        })
                })
        }
    }
        .unwrap()
}
#[export_name = "canister_update.createShipment"]
fn __canister_method_create_shipment() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let (customer_name, shipment_name, hashed_secret, channel_key, shipment_info) = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = create_shipment(
            customer_name,
            shipment_name,
            hashed_secret,
            channel_key,
            shipment_info,
        );
        ic_cdk::api::call::reply((result,))
    });
}
fn create_shipment(
    customer_name: Option<String>,
    shipment_name: String,
    hashed_secret: Vec<u8>,
    channel_key: ChannelKey,
    shipment_info: ShipmentInfo,
) -> ContractResult<u64> {
    fn inner_create_shipment(
        customer_name: Option<String>,
        shipment_name: String,
        hashed_secret: Vec<u8>,
        channel_key: ChannelKey,
        shipment_info: ShipmentInfo,
    ) -> ContractResult<u64> {
        {
            assert_whitelisted();
            let caller = ShipperKey(ActorId(ic_cdk::caller()));
            let price = shipment_info.price();
            let mut shipper = STATE
                .with_borrow_mut(|state| {
                    let shipper = match (caller.get(), customer_name) {
                        (Some(shipper), _) => shipper,
                        (None, Some(customer_name)) => {
                            RegisterActorOp::AddShipper {
                                id: caller.0,
                                name: customer_name.clone(),
                            }
                                .apply(state)
                                .map_err(|e| e.to_string())?;
                            caller.get().ok_or("Shipper could not be registered")?
                        }
                        (None, None) => {
                            ic_cdk::trap(
                                "Shipper does not exist and no name was provided",
                            );
                        }
                    };
                    ContractResult::Ok(shipper)
                })
                .map_err(|e| e.to_string())?;
            let result = STATE
                .with_borrow_mut(|state| {
                    let create_op = CreateShipmentOp::new(
                        &mut shipper,
                        hashed_secret,
                        channel_key,
                        &shipment_name,
                        &shipment_info,
                        ic_cdk::api::time(),
                    );
                    let shipment = match create_op.apply(state) {
                        Ok(shipment) => shipment,
                        Err(e) => ic_cdk::trap(&e.to_string()),
                    };
                    ContractResult::Ok(shipment)
                });
            let shipment = match result {
                Ok(shipment) => shipment,
                Err(e) => {
                    shipper.revert();
                    return Err(e);
                }
            };
            let mut balances = callers_balances();
            match balances.lock(price) {
                Ok(_) => {
                    balances.commit();
                    shipper.commit();
                    let shipment_id = *shipment.id();
                    shipment.set();
                    Ok(shipment_id)
                }
                Err(e) => {
                    shipper.revert();
                    balances.revert();
                    Err(e.to_string())
                }
            }
        }
    }
    let r = inner_create_shipment(
        customer_name,
        shipment_name,
        hashed_secret,
        channel_key,
        shipment_info,
    );
    match &r {
        Ok(_) => {}
        Err(_) => {}
    }
    r
}
pub fn encode_create_shipment_args(
    customer_name: Option<String>,
    shipment_name: String,
    hashed_secret: Vec<u8>,
    channel_key: ChannelKey,
    shipment_info: ShipmentInfo,
) -> Vec<u8> {
    use candid::Encode;
    {
        let mut builder = ::candid::ser::IDLBuilder::new();
        {
            builder
                .arg(&&customer_name)
                .and_then(|builder| {
                    builder
                        .arg(&shipment_name)
                        .and_then(|builder| {
                            builder
                                .arg(&hashed_secret)
                                .and_then(|builder| {
                                    builder
                                        .arg(&channel_key)
                                        .and_then(|builder| {
                                            builder
                                                .arg(&shipment_info)
                                                .and_then(|builder| { builder.serialize_to_vec() })
                                        })
                                })
                        })
                })
        }
    }
        .unwrap()
}
#[export_name = "canister_update.cancelShipment"]
fn __canister_method_cancel_shipment() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let (shipment,) = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = cancel_shipment(shipment);
        ic_cdk::api::call::reply((result,))
    });
}
fn cancel_shipment(
    shipment: <Shipment as apresh_store::DatabaseKeyable>::Key,
) -> ContractResult<()> {
    fn inner_cancel_shipment(shipment: &mut Shipment) -> ContractResult<()> {
        {
            assert_whitelisted();
            let caller = ShipperKey(ActorId(ic_cdk::caller()));
            let shipper = caller.get().unwrap();
            STATE
                .with_borrow_mut(|state| {
                    CancelShipmentOp::new(&shipper, shipment).apply(state)
                })
                .map_err(|e| e.to_string())?;
            Ok(())
        }
    }
    let mut shipment = shipment.get().unwrap();
    let r = inner_cancel_shipment(&mut shipment);
    match &r {
        Ok(_) => {
            shipment.commit();
        }
        Err(_) => {
            shipment.revert();
        }
    }
    r
}
pub fn encode_cancel_shipment_args(
    shipment: <Shipment as apresh_store::DatabaseKeyable>::Key,
) -> Vec<u8> {
    use candid::Encode;
    {
        let mut builder = ::candid::ser::IDLBuilder::new();
        { builder.arg(&&shipment).and_then(|builder| { builder.serialize_to_vec() }) }
    }
        .unwrap()
}
#[export_name = "canister_query.listPendingShipments"]
fn __canister_method_get_pending_shipments() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let () = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = get_pending_shipments();
        ic_cdk::api::call::reply((result,))
    });
}
fn get_pending_shipments() -> Vec<PrintableShipment> {
    Shipment::range_scan(None, None)
        .into_iter()
        .filter_map(Shipment::get)
        .filter(|shipment| *shipment.status() == ShipmentStatus::Pending)
        .map(PrintableShipment::from)
        .collect()
}
#[export_name = "canister_query.shipper_shipments"]
fn __canister_method_shipper_shipments() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let () = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = shipper_shipments();
        ic_cdk::api::call::reply((result,))
    });
}
fn shipper_shipments() -> Vec<PrintableShipment> {
    let customer_id = ActorId(ic_cdk::caller());
    Shipment::range_scan(None, None)
        .into_iter()
        .filter_map(Shipment::get)
        .filter(|shipment| *shipment.status() == ShipmentStatus::Pending)
        .filter(|shipment| *shipment.shipper_id() == customer_id)
        .filter(|shipment| !shipment.status().is_finished())
        .map(PrintableShipment::from)
        .collect()
}
#[export_name = "canister_query.carrier_shipments"]
fn __canister_method_carrier_shipments() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let () = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = carrier_shipments();
        ic_cdk::api::call::reply((result,))
    });
}
fn carrier_shipments() -> Vec<PrintableShipment> {
    let customer_id = ActorId(ic_cdk::caller());
    Shipment::range_scan(None, None)
        .into_iter()
        .filter_map(Shipment::get)
        .filter(|shipment| shipment.carrier_id() == &Some(customer_id))
        .filter(|shipment| !shipment.status().is_finished())
        .map(PrintableShipment::from)
        .collect()
}
#[export_name = "canister_query.roles"]
fn __canister_method_roles() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let () = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = roles();
        ic_cdk::api::call::reply(result)
    });
}
fn roles() -> (bool, bool) {
    let caller = ic_cdk::caller();
    let carrier = (CarrierKey(caller.into()).get()).is_some();
    let shipper: bool = (ShipperKey(caller.into()).get()).is_some();
    (carrier, shipper)
}
#[export_name = "canister_query.shipments"]
fn __canister_method_shipments() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let () = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = shipments();
        ic_cdk::api::call::reply((result,))
    });
}
fn shipments() -> Vec<PrintableShipment> {
    Shipment::range_scan(None, None)
        .into_iter()
        .filter_map(Shipment::get)
        .map(PrintableShipment::from)
        .collect()
}
#[export_name = "canister_query.shipment"]
fn __canister_method_get_shipment() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let (shipment,) = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = get_shipment(shipment);
        ic_cdk::api::call::reply((result,))
    });
}
fn get_shipment(
    shipment: <Shipment as apresh_store::DatabaseKeyable>::Key,
) -> ContractResult<PrintableShipment> {
    fn inner_get_shipment(shipment: &mut Shipment) -> ContractResult<PrintableShipment> {
        { Ok(PrintableShipment::from(&*shipment)) }
    }
    let mut shipment = shipment.get().unwrap();
    let r = inner_get_shipment(&mut shipment);
    match &r {
        Ok(_) => {
            shipment.commit();
        }
        Err(_) => {
            shipment.revert();
        }
    }
    r
}
pub fn encode_get_shipment_args(
    shipment: <Shipment as apresh_store::DatabaseKeyable>::Key,
) -> Vec<u8> {
    use candid::Encode;
    {
        let mut builder = ::candid::ser::IDLBuilder::new();
        { builder.arg(&&shipment).and_then(|builder| { builder.serialize_to_vec() }) }
    }
        .unwrap()
}
#[export_name = "canister_query.generateQr"]
fn __canister_method_generate_qr() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let (link, size) = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = generate_qr(link, size).await;
        ic_cdk::api::call::reply((result,))
    });
}
async fn generate_qr(link: String, size: usize) -> ContractResult<Vec<u8>> {
    generate(QrCodeOptions {
            gradient: false,
            link,
            size,
            transparent: false,
        })
        .map_err(|e| e.to_string())
}
#[export_name = "canister_update.lockCanister"]
fn __canister_method_lock_canister() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let () = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = lock_canister();
        ic_cdk::api::call::reply(())
    });
}
fn lock_canister() {
    assert_admin();
    CANISTER_LOCKED.with_borrow_mut(|locked| *locked = true);
}
#[export_name = "canister_update.unlockCanister"]
fn __canister_method_unlock_canister() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let () = ic_cdk::api::call::arg_data(ic_cdk::api::call::ArgDecoderConfig {
            decoding_quota: None,
            skipping_quota: Some(10000usize),
            debug: false,
        });
        let result = unlock_canister();
        ic_cdk::api::call::reply(())
    });
}
fn unlock_canister() {
    assert_admin();
    CANISTER_LOCKED.with_borrow_mut(|locked| *locked = false);
}
#[export_name = "canister_post_upgrade"]
fn __canister_method_post_upgrade() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let result = post_upgrade();
    });
}
pub fn post_upgrade() {
    CANISTER_LOCKED
        .with_borrow_mut(|locked| {
            *locked = true;
        });
}
fn __export_service() -> String {
    use ::candid::types::{CandidType, Function, Type, TypeInner};
    let mut service = Vec::<(String, Type)>::new();
    let mut env = ::candid::types::internal::TypeContainer::new();
    {
        let mut args = Vec::new();
        args.push(env.add::<Principal>());
        let mut rets = Vec::new();
        let func = Function {
            args,
            rets,
            modes: ::alloc::vec::Vec::new(),
        };
        service.push(("addWhitelisted".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        args.push(env.add::<Vec<u8>>());
        args.push(env.add::<<Shipment as apresh_store::DatabaseKeyable>::Key>());
        let mut rets = Vec::new();
        rets.push(env.add::<ContractResult<()>>());
        let func = Function {
            args,
            rets,
            modes: ::alloc::vec::Vec::new(),
        };
        service.push(("add_message".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        let mut rets = Vec::new();
        rets.push(env.add::<u64>());
        rets.push(env.add::<u64>());
        let func = Function {
            args,
            rets,
            modes: <[_]>::into_vec(
                ::alloc::boxed::box_new([::candid::types::FuncMode::Query]),
            ),
        };
        service.push(("balance".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        args.push(env.add::<Option<String>>());
        args.push(env.add::<<Shipment as apresh_store::DatabaseKeyable>::Key>());
        args.push(env.add::<ChannelKey>());
        let mut rets = Vec::new();
        rets.push(env.add::<ContractResult<()>>());
        let func = Function {
            args,
            rets,
            modes: ::alloc::vec::Vec::new(),
        };
        service.push(("buyShipment".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        args.push(env.add::<<Shipment as apresh_store::DatabaseKeyable>::Key>());
        let mut rets = Vec::new();
        rets.push(env.add::<ContractResult<()>>());
        let func = Function {
            args,
            rets,
            modes: ::alloc::vec::Vec::new(),
        };
        service.push(("cancelShipment".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        let mut rets = Vec::new();
        rets.push(env.add::<Vec<PrintableShipment>>());
        let func = Function {
            args,
            rets,
            modes: <[_]>::into_vec(
                ::alloc::boxed::box_new([::candid::types::FuncMode::Query]),
            ),
        };
        service.push(("carrier_shipments".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        args.push(env.add::<Option<String>>());
        args.push(env.add::<String>());
        args.push(env.add::<Vec<u8>>());
        args.push(env.add::<ChannelKey>());
        args.push(env.add::<ShipmentInfo>());
        let mut rets = Vec::new();
        rets.push(env.add::<ContractResult<u64>>());
        let func = Function {
            args,
            rets,
            modes: ::alloc::vec::Vec::new(),
        };
        service.push(("createShipment".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        args.push(env.add::<u64>());
        let mut rets = Vec::new();
        rets.push(env.add::<ContractResult<()>>());
        let func = Function {
            args,
            rets,
            modes: ::alloc::vec::Vec::new(),
        };
        service.push(("deposit".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        args.push(env.add::<<Shipment as apresh_store::DatabaseKeyable>::Key>());
        args.push(env.add::<Option<String>>());
        let mut rets = Vec::new();
        rets.push(env.add::<ContractResult<()>>());
        let func = Function {
            args,
            rets,
            modes: ::alloc::vec::Vec::new(),
        };
        service.push(("finalizeShipment".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        args.push(env.add::<String>());
        args.push(env.add::<usize>());
        let mut rets = Vec::new();
        rets.push(env.add::<ContractResult<Vec<u8>>>());
        let func = Function {
            args,
            rets,
            modes: <[_]>::into_vec(
                ::alloc::boxed::box_new([::candid::types::FuncMode::Query]),
            ),
        };
        service.push(("generateQr".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        let mut rets = Vec::new();
        rets.push(env.add::<u64>());
        let func = Function {
            args,
            rets,
            modes: <[_]>::into_vec(
                ::alloc::boxed::box_new([::candid::types::FuncMode::Query]),
            ),
        };
        service.push(("getTransferFee".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        let mut rets = Vec::new();
        rets.push(env.add::<bool>());
        let func = Function {
            args,
            rets,
            modes: <[_]>::into_vec(
                ::alloc::boxed::box_new([::candid::types::FuncMode::Query]),
            ),
        };
        service.push(("is_mainnet".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        let mut rets = Vec::new();
        rets.push(env.add::<Vec<PrintableShipment>>());
        let func = Function {
            args,
            rets,
            modes: <[_]>::into_vec(
                ::alloc::boxed::box_new([::candid::types::FuncMode::Query]),
            ),
        };
        service.push(("listPendingShipments".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        let mut rets = Vec::new();
        let func = Function {
            args,
            rets,
            modes: ::alloc::vec::Vec::new(),
        };
        service.push(("lockCanister".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        args.push(env.add::<<Shipment as apresh_store::DatabaseKeyable>::Key>());
        let mut rets = Vec::new();
        rets.push(env.add::<ContractResult<Channel>>());
        let func = Function {
            args,
            rets,
            modes: <[_]>::into_vec(
                ::alloc::boxed::box_new([::candid::types::FuncMode::Query]),
            ),
        };
        service.push(("read_channel".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        let mut rets = Vec::new();
        rets.push(env.add::<bool>());
        rets.push(env.add::<bool>());
        let func = Function {
            args,
            rets,
            modes: <[_]>::into_vec(
                ::alloc::boxed::box_new([::candid::types::FuncMode::Query]),
            ),
        };
        service.push(("roles".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        args.push(env.add::<u64>());
        let mut rets = Vec::new();
        let func = Function {
            args,
            rets,
            modes: ::alloc::vec::Vec::new(),
        };
        service.push(("setTransferFee".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        args.push(env.add::<<Shipment as apresh_store::DatabaseKeyable>::Key>());
        let mut rets = Vec::new();
        rets.push(env.add::<ContractResult<PrintableShipment>>());
        let func = Function {
            args,
            rets,
            modes: <[_]>::into_vec(
                ::alloc::boxed::box_new([::candid::types::FuncMode::Query]),
            ),
        };
        service.push(("shipment".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        let mut rets = Vec::new();
        rets.push(env.add::<Vec<PrintableShipment>>());
        let func = Function {
            args,
            rets,
            modes: <[_]>::into_vec(
                ::alloc::boxed::box_new([::candid::types::FuncMode::Query]),
            ),
        };
        service.push(("shipments".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        let mut rets = Vec::new();
        rets.push(env.add::<Vec<PrintableShipment>>());
        let func = Function {
            args,
            rets,
            modes: <[_]>::into_vec(
                ::alloc::boxed::box_new([::candid::types::FuncMode::Query]),
            ),
        };
        service.push(("shipper_shipments".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        let mut rets = Vec::new();
        let func = Function {
            args,
            rets,
            modes: ::alloc::vec::Vec::new(),
        };
        service.push(("unlockCanister".to_string(), TypeInner::Func(func).into()));
    }
    {
        let mut args = Vec::new();
        args.push(env.add::<u64>());
        let mut rets = Vec::new();
        rets.push(env.add::<ContractResult<()>>());
        let func = Function {
            args,
            rets,
            modes: ::alloc::vec::Vec::new(),
        };
        service.push(("withdraw".to_string(), TypeInner::Func(func).into()));
    }
    service.sort_unstable_by_key(|(name, _)| name.clone());
    let ty = TypeInner::Service(service).into();
    let mut init_args = Vec::new();
    let actor = Some(TypeInner::Class(init_args, ty).into());
    let result = ::candid::pretty::candid::compile(&env.env, &actor);
    ::alloc::__export::must_use({
        let res = ::alloc::fmt::format(format_args!("{0}", result));
        res
    })
}
#[no_mangle]
pub fn get_candid_pointer() -> *mut std::os::raw::c_char {
    let c_string = std::ffi::CString::new(__export_service()).unwrap();
    c_string.into_raw()
}
