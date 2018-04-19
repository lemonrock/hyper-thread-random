// This file is part of hyper-thread-random. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/hyper-thread-random/master/COPYRIGHT. No part of hyper-thread-random, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of hyper-thread-random. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/hyper-thread-random/master/COPYRIGHT.


/// Generates a random u16 for the current hyper thread.
#[cfg(all(target_arch = "x86", target_feature = "rdrnd"))]
#[inline(always)]
pub fn generate_hyper_thread_safe_random_u16() -> u16
{
	generate!(u16, ::std::arch::x86::_rdrand64_step)
}

/// Generates a random u16 for the current hyper thread.
#[cfg(all(target_arch = "x86_64", target_feature = "rdrnd"))]
#[inline(always)]
pub fn generate_hyper_thread_safe_random_u16() -> u16
{
	generate!(u16, ::std::arch::x86_64::_rdrand64_step)
}

/// Generates a random u16 for the current hyper thread.
#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "rdrnd")))]
pub fn generate_hyper_thread_safe_random_u16() -> u16
{
	thread_rng().next_u32() as u16
}
