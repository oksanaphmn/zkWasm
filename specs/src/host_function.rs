use serde::Serialize;

use crate::{external_host_call_table::ExternalHostCallSignature, types::ValueType};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Signature {
    pub params: Vec<ValueType>,
    pub return_type: Option<ValueType>,
}

#[derive(Debug)]
pub enum Error {
    DuplicateRegister,
}

#[derive(Debug, Clone)]
pub enum HostFunctionDesc {
    Internal {
        name: String,
        op_index_in_plugin: usize,
        plugin: HostPlugin,
    },
    External {
        name: String,
        op: usize,
        sig: ExternalHostCallSignature,
    },
}

#[derive(Clone, Debug, Serialize, Copy, PartialEq, Eq, Hash)]
pub enum HostPlugin {
    HostInput = 0,
    Sha256,
    Require,
}
