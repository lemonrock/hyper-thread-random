// This file is part of hyper-thread-random. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/hyper-thread-random/master/COPYRIGHT. No part of hyper-thread-random, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of hyper-thread-random. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/hyper-thread-random/master/COPYRIGHT.


/// Generates a random usize for the current hyper thread.
#[cfg(target_pointer_size = "16")]
#[inline(always)]
pub fn generate_hyper_thread_safe_random_usize() -> usize
{
	generate_hyper_thread_safe_random_u16() as usize
}

/// Generates a random usize for the current hyper thread.
#[cfg(target_pointer_size = "32")]
#[inline(always)]
pub fn generate_hyper_thread_safe_random_usize() -> usize
{
	generate_hyper_thread_safe_random_u32() as usize
}

/// Generates a random usize for the current hyper thread.
#[cfg(target_pointer_size = "64")]
#[inline(always)]
pub fn generate_hyper_thread_safe_random_usize() -> usize
{
	generate_hyper_thread_safe_random_u32() as usize
}
