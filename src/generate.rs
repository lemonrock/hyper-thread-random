// This file is part of hyper-thread-random. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/hyper-thread-random/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of hyper-thread-random. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/hyper-thread-random/master/COPYRIGHT.


#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "rdrand"))]
macro_rules! generate
{
	($unsigned_integer: ty, $func: path) =>
	{
		{
			//#[target_feature(enable = "rdrnd")]
			#[inline(always)]
			unsafe fn generate() -> $unsigned_integer
			{
				let mut random_value = ::std::mem::uninitialized();
				
				loop
				{
					let success = $func(&mut random_value);
					if success != 0
					{
						return random_value
					}
				}
			}
			
			unsafe { generate() }
		}
	}
}
