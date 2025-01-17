use crate::errors::{ErrorKind, InputValue, LocItem, ValError, ValResult};

use super::datetime::{
    bytes_as_date, bytes_as_datetime, bytes_as_time, bytes_as_timedelta, float_as_datetime, float_as_duration,
    float_as_time, int_as_datetime, int_as_duration, int_as_time, EitherDate, EitherDateTime, EitherTime,
};
use super::shared::{float_as_int, int_as_bool, str_as_bool, str_as_int};
use super::{EitherBytes, EitherString, EitherTimedelta, GenericMapping, GenericSequence, Input, JsonInput};

impl<'a> Input<'a> for JsonInput {
    /// This is required by since JSON object keys are always strings, I don't think it can be called
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn as_loc_item(&self) -> LocItem {
        match self {
            JsonInput::Int(i) => LocItem::I(*i as usize),
            JsonInput::String(s) => s.as_str().into(),
            v => format!("{:?}", v).into(),
        }
    }

    fn as_error_value(&'a self) -> InputValue<'a> {
        InputValue::JsonInput(self)
    }

    fn is_none(&self) -> bool {
        matches!(self, JsonInput::Null)
    }

    fn strict_str(&'a self) -> ValResult<EitherString<'a>> {
        match self {
            JsonInput::String(s) => Ok(s.as_str().into()),
            _ => Err(ValError::new(ErrorKind::StrType, self)),
        }
    }
    fn lax_str(&'a self) -> ValResult<EitherString<'a>> {
        match self {
            JsonInput::String(s) => Ok(s.as_str().into()),
            JsonInput::Int(int) => Ok(int.to_string().into()),
            JsonInput::Float(float) => Ok(float.to_string().into()),
            _ => Err(ValError::new(ErrorKind::StrType, self)),
        }
    }

    fn validate_bytes(&'a self, _strict: bool) -> ValResult<EitherBytes<'a>> {
        match self {
            JsonInput::String(s) => Ok(s.as_bytes().into()),
            _ => Err(ValError::new(ErrorKind::BytesType, self)),
        }
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_bytes(&'a self) -> ValResult<EitherBytes<'a>> {
        self.validate_bytes(false)
    }

    fn strict_bool(&self) -> ValResult<bool> {
        match self {
            JsonInput::Bool(b) => Ok(*b),
            _ => Err(ValError::new(ErrorKind::BoolType, self)),
        }
    }
    fn lax_bool(&self) -> ValResult<bool> {
        match self {
            JsonInput::Bool(b) => Ok(*b),
            JsonInput::String(s) => str_as_bool(self, s),
            JsonInput::Int(int) => int_as_bool(self, *int),
            JsonInput::Float(float) => match float_as_int(self, *float) {
                Ok(int) => int_as_bool(self, int),
                _ => Err(ValError::new(ErrorKind::BoolType, self)),
            },
            _ => Err(ValError::new(ErrorKind::BoolType, self)),
        }
    }

    fn strict_int(&self) -> ValResult<i64> {
        match self {
            JsonInput::Int(i) => Ok(*i),
            _ => Err(ValError::new(ErrorKind::IntType, self)),
        }
    }
    fn lax_int(&self) -> ValResult<i64> {
        match self {
            JsonInput::Bool(b) => match *b {
                true => Ok(1),
                false => Ok(0),
            },
            JsonInput::Int(i) => Ok(*i),
            JsonInput::Float(f) => float_as_int(self, *f),
            JsonInput::String(str) => str_as_int(self, str),
            _ => Err(ValError::new(ErrorKind::IntType, self)),
        }
    }

    fn strict_float(&self) -> ValResult<f64> {
        match self {
            JsonInput::Float(f) => Ok(*f),
            JsonInput::Int(i) => Ok(*i as f64),
            _ => Err(ValError::new(ErrorKind::FloatType, self)),
        }
    }
    fn lax_float(&self) -> ValResult<f64> {
        match self {
            JsonInput::Bool(b) => match *b {
                true => Ok(1.0),
                false => Ok(0.0),
            },
            JsonInput::Float(f) => Ok(*f),
            JsonInput::Int(i) => Ok(*i as f64),
            JsonInput::String(str) => match str.parse() {
                Ok(i) => Ok(i),
                Err(_) => Err(ValError::new(ErrorKind::FloatParsing, self)),
            },
            _ => Err(ValError::new(ErrorKind::FloatType, self)),
        }
    }

    fn validate_dict(&'a self, _strict: bool) -> ValResult<GenericMapping<'a>> {
        match self {
            JsonInput::Object(dict) => Ok(dict.into()),
            _ => Err(ValError::new(ErrorKind::DictType, self)),
        }
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_dict(&'a self) -> ValResult<GenericMapping<'a>> {
        self.validate_dict(false)
    }

    fn validate_list(&'a self, _strict: bool) -> ValResult<GenericSequence<'a>> {
        match self {
            JsonInput::Array(a) => Ok(a.into()),
            _ => Err(ValError::new(ErrorKind::ListType, self)),
        }
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_list(&'a self) -> ValResult<GenericSequence<'a>> {
        self.validate_list(false)
    }

    fn validate_tuple(&'a self, _strict: bool) -> ValResult<GenericSequence<'a>> {
        // just as in set's case, List has to be allowed
        match self {
            JsonInput::Array(a) => Ok(a.into()),
            _ => Err(ValError::new(ErrorKind::TupleType, self)),
        }
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_tuple(&'a self) -> ValResult<GenericSequence<'a>> {
        self.validate_tuple(false)
    }

    fn validate_set(&'a self, _strict: bool) -> ValResult<GenericSequence<'a>> {
        // we allow a list here since otherwise it would be impossible to create a set from JSON
        match self {
            JsonInput::Array(a) => Ok(a.into()),
            _ => Err(ValError::new(ErrorKind::SetType, self)),
        }
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_set(&'a self) -> ValResult<GenericSequence<'a>> {
        self.validate_set(false)
    }

    fn validate_frozenset(&'a self, _strict: bool) -> ValResult<GenericSequence<'a>> {
        // we allow a list here since otherwise it would be impossible to create a frozenset from JSON
        match self {
            JsonInput::Array(a) => Ok(a.into()),
            _ => Err(ValError::new(ErrorKind::FrozenSetType, self)),
        }
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_frozenset(&'a self) -> ValResult<GenericSequence<'a>> {
        self.validate_frozenset(false)
    }

    fn validate_date(&self, _strict: bool) -> ValResult<EitherDate> {
        match self {
            JsonInput::String(v) => bytes_as_date(self, v.as_bytes()),
            _ => Err(ValError::new(ErrorKind::DateType, self)),
        }
    }
    // NO custom `lax_date` implementation, if strict_date fails, the validator will fallback to lax_datetime
    // then check there's no remainder
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_date(&self) -> ValResult<EitherDate> {
        self.validate_date(false)
    }

    fn strict_time(&self) -> ValResult<EitherTime> {
        match self {
            JsonInput::String(v) => bytes_as_time(self, v.as_bytes()),
            _ => Err(ValError::new(ErrorKind::TimeType, self)),
        }
    }
    fn lax_time(&self) -> ValResult<EitherTime> {
        match self {
            JsonInput::String(v) => bytes_as_time(self, v.as_bytes()),
            JsonInput::Int(v) => int_as_time(self, *v, 0),
            JsonInput::Float(v) => float_as_time(self, *v),
            _ => Err(ValError::new(ErrorKind::TimeType, self)),
        }
    }

    fn strict_datetime(&self) -> ValResult<EitherDateTime> {
        match self {
            JsonInput::String(v) => bytes_as_datetime(self, v.as_bytes()),
            _ => Err(ValError::new(ErrorKind::DateTimeType, self)),
        }
    }
    fn lax_datetime(&self) -> ValResult<EitherDateTime> {
        match self {
            JsonInput::String(v) => bytes_as_datetime(self, v.as_bytes()),
            JsonInput::Int(v) => int_as_datetime(self, *v, 0),
            JsonInput::Float(v) => float_as_datetime(self, *v),
            _ => Err(ValError::new(ErrorKind::DateTimeType, self)),
        }
    }

    fn strict_timedelta(&self) -> ValResult<EitherTimedelta> {
        match self {
            JsonInput::String(v) => bytes_as_timedelta(self, v.as_bytes()),
            _ => Err(ValError::new(ErrorKind::TimeDeltaType, self)),
        }
    }
    fn lax_timedelta(&self) -> ValResult<EitherTimedelta> {
        match self {
            JsonInput::String(v) => bytes_as_timedelta(self, v.as_bytes()),
            JsonInput::Int(v) => Ok(int_as_duration(*v).into()),
            JsonInput::Float(v) => Ok(float_as_duration(*v).into()),
            _ => Err(ValError::new(ErrorKind::TimeDeltaType, self)),
        }
    }
}

/// Required for Dict keys so the string can behave like an Input
impl<'a> Input<'a> for String {
    fn as_loc_item(&self) -> LocItem {
        self.to_string().into()
    }

    fn as_error_value(&'a self) -> InputValue<'a> {
        InputValue::String(self)
    }

    #[cfg_attr(has_no_coverage, no_coverage)]
    fn is_none(&self) -> bool {
        false
    }

    fn validate_str(&'a self, _strict: bool) -> ValResult<EitherString<'a>> {
        Ok(self.as_str().into())
    }
    fn strict_str(&'a self) -> ValResult<EitherString<'a>> {
        self.validate_str(false)
    }

    fn validate_bytes(&'a self, _strict: bool) -> ValResult<EitherBytes<'a>> {
        Ok(self.as_bytes().into())
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_bytes(&'a self) -> ValResult<EitherBytes<'a>> {
        self.validate_bytes(false)
    }

    fn strict_bool(&self) -> ValResult<bool> {
        Err(ValError::new(ErrorKind::BoolType, self))
    }
    fn lax_bool(&self) -> ValResult<bool> {
        str_as_bool(self, self)
    }

    fn strict_int(&self) -> ValResult<i64> {
        Err(ValError::new(ErrorKind::IntType, self))
    }
    fn lax_int(&self) -> ValResult<i64> {
        match self.parse() {
            Ok(i) => Ok(i),
            Err(_) => Err(ValError::new(ErrorKind::IntParsing, self)),
        }
    }

    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_float(&self) -> ValResult<f64> {
        Err(ValError::new(ErrorKind::FloatType, self))
    }
    fn lax_float(&self) -> ValResult<f64> {
        match self.parse() {
            Ok(i) => Ok(i),
            Err(_) => Err(ValError::new(ErrorKind::FloatParsing, self)),
        }
    }

    #[cfg_attr(has_no_coverage, no_coverage)]
    fn validate_dict(&'a self, _strict: bool) -> ValResult<GenericMapping<'a>> {
        Err(ValError::new(ErrorKind::DictType, self))
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_dict(&'a self) -> ValResult<GenericMapping<'a>> {
        self.validate_dict(false)
    }

    #[cfg_attr(has_no_coverage, no_coverage)]
    fn validate_list(&'a self, _strict: bool) -> ValResult<GenericSequence<'a>> {
        Err(ValError::new(ErrorKind::ListType, self))
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_list(&'a self) -> ValResult<GenericSequence<'a>> {
        self.validate_list(false)
    }

    #[cfg_attr(has_no_coverage, no_coverage)]
    fn validate_tuple(&'a self, _strict: bool) -> ValResult<GenericSequence<'a>> {
        Err(ValError::new(ErrorKind::TupleType, self))
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_tuple(&'a self) -> ValResult<GenericSequence<'a>> {
        self.validate_tuple(false)
    }

    #[cfg_attr(has_no_coverage, no_coverage)]
    fn validate_set(&'a self, _strict: bool) -> ValResult<GenericSequence<'a>> {
        Err(ValError::new(ErrorKind::SetType, self))
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_set(&'a self) -> ValResult<GenericSequence<'a>> {
        self.validate_set(false)
    }

    #[cfg_attr(has_no_coverage, no_coverage)]
    fn validate_frozenset(&'a self, _strict: bool) -> ValResult<GenericSequence<'a>> {
        Err(ValError::new(ErrorKind::FrozenSetType, self))
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_frozenset(&'a self) -> ValResult<GenericSequence<'a>> {
        self.validate_frozenset(false)
    }

    fn validate_date(&self, _strict: bool) -> ValResult<EitherDate> {
        bytes_as_date(self, self.as_bytes())
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_date(&self) -> ValResult<EitherDate> {
        self.validate_date(false)
    }

    fn validate_time(&self, _strict: bool) -> ValResult<EitherTime> {
        bytes_as_time(self, self.as_bytes())
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_time(&self) -> ValResult<EitherTime> {
        self.validate_time(false)
    }

    fn validate_datetime(&self, _strict: bool) -> ValResult<EitherDateTime> {
        bytes_as_datetime(self, self.as_bytes())
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_datetime(&self) -> ValResult<EitherDateTime> {
        self.validate_datetime(false)
    }

    fn validate_timedelta(&self, _strict: bool) -> ValResult<EitherTimedelta> {
        bytes_as_timedelta(self, self.as_bytes())
    }
    #[cfg_attr(has_no_coverage, no_coverage)]
    fn strict_timedelta(&self) -> ValResult<EitherTimedelta> {
        self.validate_timedelta(false)
    }
}
