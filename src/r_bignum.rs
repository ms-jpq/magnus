use std::ops::Deref;

use crate::{
    protect,
    r_basic::RBasic,
    ruby_sys::{rb_ll2inum, rb_num2ll, rb_num2ull, rb_ull2inum, ruby_value_type, VALUE},
    value::{Fixnum, Qnil, Value},
    ProtectState,
};

#[repr(transparent)]
pub struct RBignum(VALUE);

impl RBignum {
    /// # Safety
    ///
    /// val must not have been GC'd, return value must be kept on stack or
    /// otherwise protected from the GC.
    pub unsafe fn from_value(val: &Value) -> Option<Self> {
        let r_basic = RBasic::from_value(val)?;
        (r_basic.builtin_type() == ruby_value_type::RUBY_T_BIGNUM).then(|| Self(val.into_inner()))
    }

    pub fn from_i64(n: i64) -> Result<Self, Fixnum> {
        let val = unsafe { Value::new(rb_ll2inum(n)) };
        unsafe { RBignum::from_value(&val) }.ok_or_else(|| {
            Fixnum::from_value(&val).expect("i64 should convert to fixnum or bignum")
        })
    }

    pub fn from_u64(n: u64) -> Result<Self, Fixnum> {
        let val = unsafe { Value::new(rb_ull2inum(n)) };
        unsafe { RBignum::from_value(&val) }.ok_or_else(|| {
            Fixnum::from_value(&val).expect("u64 should convert to fixnum or bignum")
        })
    }

    pub fn to_i64(&self) -> Result<i64, ProtectState> {
        let mut res = 0;
        unsafe {
            protect(|| {
                res = rb_num2ll(self.into_inner());
                *Qnil::new()
            })?;
        }
        Ok(res)
    }

    pub fn to_u64(&self) -> Result<u64, ProtectState> {
        let mut res = 0;
        unsafe {
            protect(|| {
                res = rb_num2ull(self.into_inner());
                *Qnil::new()
            })?;
        }
        Ok(res)
    }
}

impl Deref for RBignum {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        let self_ptr = self as *const Self;
        let value_ptr = self_ptr as *const Self::Target;
        // we just got this pointer from &self, so we know it's valid to deref
        unsafe { &*value_ptr }
    }
}
