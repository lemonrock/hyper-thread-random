// This file is part of hyper-thread-random. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/hyper-thread-random/master/COPYRIGHT. No part of hyper-thread-random, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of hyper-thread-random. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/hyper-thread-random/master/COPYRIGHT.


#![deny(missing_docs)]
#![feature(cfg_target_feature)]


//! #hyper-thread-random
//! Provides hyper-thread local random number generators optimized for recent Intel x86-64 chips with the `RDRAND` instruction; falls back to rand crate for others.


#[cfg(not(all(target_feature = "+rdrnd", any(target_arch = "x86", target_arch = "x86_64"))))] extern crate rand;


include!("generate_hyper_thread_safe_random_u64.rs");
include!("generate_hyper_thread_safe_random_usize.rs");
