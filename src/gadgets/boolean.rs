use plonk::cs::constraint_system::Variable;

/// Represents a Variable that has already
/// been constrained to be either one or zero.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct BoolVar(pub(crate) Variable);

impl From<Variable> for BoolVar {
    fn from(v: Variable) -> BoolVar {
        BoolVar(v)
    }
}

impl Into<Variable> for BoolVar {
    fn into(self) -> Variable {
        self.0
    }
}
