

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Parameter {
    pub(crate) name: String,
    pub(crate) type_: ParameterType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParameterType {
    Unknown,
    Integer,
}