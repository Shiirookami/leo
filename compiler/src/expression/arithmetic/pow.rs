// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! Enforces an arithmetic `**` operator in a resolved Leo program.

use crate::{errors::ExpressionError, value::ConstrainedValue, GroupType};
use leo_ast::Span;

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

pub fn enforce_pow<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: ConstrainedValue<F, G>,
    right: ConstrainedValue<F, G>,
    span: &Span,
) -> Result<ConstrainedValue<F, G>, ExpressionError> {
    match (left, right) {
        (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
            Ok(ConstrainedValue::Integer(num_1.pow(cs, num_2, span)?))
        }
        (ConstrainedValue::Unresolved(string), val_2) => {
            let val_1 = ConstrainedValue::from_other(string, &val_2, span)?;
            enforce_pow(cs, val_1, val_2, span)
        }
        (val_1, ConstrainedValue::Unresolved(string)) => {
            let val_2 = ConstrainedValue::from_other(string, &val_1, span)?;
            enforce_pow(cs, val_1, val_2, span)
        }
        (val_1, val_2) => Err(ExpressionError::incompatible_types(
            format!("{} ** {}", val_1, val_2,),
            span.to_owned(),
        )),
    }
}
