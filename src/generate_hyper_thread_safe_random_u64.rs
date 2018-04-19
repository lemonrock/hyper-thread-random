// This file is part of hyper-thread-random. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/hyper-thread-random/master/COPYRIGHT. No part of hyper-thread-random, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of hyper-thread-random. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/hyper-thread-random/master/COPYRIGHT.


/// Generates a random u64 for the current hyper thread.
#[cfg(all(target_feature = "rdrnd", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
pub fn generate_hyper_thread_safe_random_u64() -> u64
{
	// See https://github.com/rust-lang/rust/tree/master/src/etc/platform-intrinsics/x86
	extern "platform-intrinsic"
	{
		#[inline(always)]
		fn x86_rdrand64_step() -> (u64, i32);
	}

	#[target_feature(enable = "rdrnd")]
	unsafe fn generate_hyper_thread_safe_random_u64_target_feature() -> u64
	{
		loop
		{
			let (random_value, success) = x86_rdrand64_step();
			if success != 0
			{
				return random_value
			}
		}
	}

	unsafe { generate_hyper_thread_safe_random_u64_target_feature() }
}

/// Generates a random u64 for the current hyper thread.
#[cfg(not(all(target_feature = "rdrnd", any(target_arch = "x86", target_arch = "x86_64"))))]
pub fn generate_hyper_thread_safe_random_u64() -> u64
{
	// Not made module-level imports as the `unused import` lint mistakenly lists them.
	use ::rand::Rng;
	use ::rand::thread_rng;
	
	thread_rng().next_u64()
}
