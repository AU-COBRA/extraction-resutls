#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::marker::PhantomData;

fn __nat_succ(x: u64) -> u64 {
  x.checked_add(1).unwrap()
}

macro_rules! __nat_elim {
    ($zcase:expr, $pred:ident, $scase:expr, $val:expr) => {
        { let v = $val;
        if v == 0 { $zcase } else { let $pred = v - 1; $scase } }
    }
}

macro_rules! __andb { ($b1:expr, $b2:expr) => { $b1 && $b2 } }
macro_rules! __orb { ($b1:expr, $b2:expr) => { $b1 || $b2 } }

fn __pos_onebit(x: u64) -> u64 {
  x.checked_mul(2).unwrap() + 1
}

fn __pos_zerobit(x: u64) -> u64 {
  x.checked_mul(2).unwrap()
}

macro_rules! __pos_elim {
    ($p:ident, $onebcase:expr, $p2:ident, $zerobcase:expr, $onecase:expr, $val:expr) => {
        {
            let n = $val;
            if n == 1 {
                $onecase
            } else if (n & 1) == 0 {
                let $p2 = n >> 1;
                $zerobcase
            } else {
                let $p = n >> 1;
                $onebcase
            }
        }
    }
}

fn __Z_frompos(z: u64) -> i64 {
  use std::convert::TryFrom;

  i64::try_from(z).unwrap()
}

fn __Z_fromneg(z : u64) -> i64 {
  use std::convert::TryFrom;

  i64::try_from(z).unwrap().checked_neg().unwrap()
}

macro_rules! __Z_elim {
    ($zero_case:expr, $p:ident, $pos_case:expr, $p2:ident, $neg_case:expr, $val:expr) => {
        {
            let n = $val;
            if n == 0 {
                $zero_case
            } else if n < 0 {
                let $p2 = n.unsigned_abs();
                $neg_case
            } else {
                let $p = n as u64;
                $pos_case
            }
        }
    }
}

fn __N_frompos(z: u64) -> u64 {
  z
}

macro_rules! __N_elim {
    ($zero_case:expr, $p:ident, $pos_case:expr, $val:expr) => {
        { let $p = $val; if $p == 0 { $zero_case } else { $pos_case } }
    }
}

type __pair<A, B> = (A, B);

macro_rules! __pair_elim {
    ($fst:ident, $snd:ident, $body:expr, $p:expr) => {
        { let ($fst, $snd) = $p; $body }
    }
}

fn __mk_pair<A: Copy, B: Copy>(a: A, b: B) -> __pair<A, B> { (a, b) }

fn hint_app<TArg, TRet>(f: &dyn Fn(TArg) -> TRet) -> &dyn Fn(TArg) -> TRet {
  f
}

struct Program {
  __alloc: bumpalo::Bump,
}

impl<'a> Program {
fn new() -> Self {
  Program {
    __alloc: bumpalo::Bump::new(),
  }
}

fn alloc<T>(&'a self, t: T) -> &'a T {
  self.__alloc.alloc(t)
}

fn closure<TArg, TRet>(&'a self, F: impl Fn(TArg) -> TRet + 'a) -> &'a dyn Fn(TArg) -> TRet {
  self.__alloc.alloc(F)
}

fn ConCert_Extraction_InternalFix_ack(&'a self, n: u64, m: u64) -> u64 {
  __nat_elim!(
    {
      __nat_succ(
        m)
    },
    p, {
      let ackn = {
        let ackn = self.alloc(std::cell::Cell::new(None));
        ackn.set(Some(
          self.closure(move |m2| {
            __nat_elim!(
              {
                self.ConCert_Extraction_InternalFix_ack(
                  p,
                  __nat_succ(
                    0))
              },
              q, {
                self.ConCert_Extraction_InternalFix_ack(
                  p,
                  hint_app(ackn.get().unwrap())(q))
              },
              m2)
          })));
        ackn.get().unwrap()
      };
      hint_app(ackn)(m)
    },
    n)
}
fn ConCert_Extraction_InternalFix_ack__curried(&'a self) -> &'a dyn Fn(u64) -> &'a dyn Fn(u64) -> u64 {
  self.closure(move |n| {
    self.closure(move |m| {
      self.ConCert_Extraction_InternalFix_ack(
        n,
        m)
    })
  })
}
}
fn main() {
  println!("{:?}", Program::new().ConCert_Extraction_InternalFix_ack(3,4))
}