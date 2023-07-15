//! This module implements system clocks.

use crate::errno::EResult;
use crate::time::unit::ClockIdT;
use crate::time::unit::TimeUnit;
use crate::time::Timestamp;
use crate::time::TimestampScale;
use core::cmp::max;
use core::sync::atomic;
use core::sync::atomic::AtomicU32;

/// System clock ID
pub const CLOCK_REALTIME: ClockIdT = 0;
/// System clock ID
pub const CLOCK_MONOTONIC: ClockIdT = 1;
/// System clock ID
pub const CLOCK_PROCESS_CPUTIME_ID: ClockIdT = 2;
/// System clock ID
pub const CLOCK_THREAD_CPUTIME_ID: ClockIdT = 3;
/// System clock ID
pub const CLOCK_MONOTONIC_RAW: ClockIdT = 4;
/// System clock ID
pub const CLOCK_REALTIME_COARSE: ClockIdT = 5;
/// System clock ID
pub const CLOCK_MONOTONIC_COARSE: ClockIdT = 6;
/// System clock ID
pub const CLOCK_BOOTTIME: ClockIdT = 7;
/// System clock ID
pub const CLOCK_REALTIME_ALARM: ClockIdT = 8;
/// System clock ID
pub const CLOCK_BOOTTIME_ALARM: ClockIdT = 9;
/// System clock ID
pub const CLOCK_SGI_CYCLE: ClockIdT = 10;
/// System clock ID
pub const CLOCK_TAI: ClockIdT = 11;

// TODO allow accessing clocks through an address shared with userspace (vDSO)
// TODO for timestamps, use u64, or a structure?

/// The current timestamp of the real time clock, in nanoseconds.
static REALTIME: AtomicU32 = AtomicU32::new(0);
/// On time adjustement, this value is updated with the previous value of the real time clock so
/// that it can be used if the clock went backwards in time.
static MONOTONIC: AtomicU32 = AtomicU32::new(0);
/// The time elapsed since boot time, in nanoseconds.
static BOOTTIME: AtomicU32 = AtomicU32::new(0);

/// Updates clocks with the given delta value in nanoseconds.
pub fn update(delta: Timestamp) {
	REALTIME.fetch_add(delta as _, atomic::Ordering::Relaxed);
	MONOTONIC.fetch_add(delta as _, atomic::Ordering::Relaxed);
	BOOTTIME.fetch_add(delta as _, atomic::Ordering::Relaxed);
}

/// Returns the current timestamp according to the clock with the given ID.
///
/// Arguments:
/// - `clk` is the ID of the clock to use.
/// - `scale` is the scale of the timestamp to return.
///
/// If the clock is invalid, the function returns an error.
pub fn current_time(clk: ClockIdT, scale: TimestampScale) -> EResult<Timestamp> {
	// TODO implement all clocks
	let raw_ts = match clk {
		CLOCK_REALTIME | CLOCK_REALTIME_ALARM => REALTIME.load(atomic::Ordering::Relaxed),
		CLOCK_MONOTONIC => {
			let realtime = REALTIME.load(atomic::Ordering::Relaxed);
			let monotonic = MONOTONIC.load(atomic::Ordering::Relaxed);

			max(realtime, monotonic)
		}
		CLOCK_BOOTTIME | CLOCK_BOOTTIME_ALARM => BOOTTIME.load(atomic::Ordering::Relaxed),

		_ => return Err(errno!(EINVAL)),
	};

	Ok(TimestampScale::convert(
		raw_ts as _,
		TimestampScale::Nanosecond,
		scale,
	))
}

/// Returns the current timestamp according to the clock with the given ID.
///
/// Arguments:
/// - `clk` is the ID of the clock to use.
/// - `scale` is the scale of the timestamp to return.
///
/// If the clock is invalid, the function returns an error.
pub fn current_time_struct<T: TimeUnit>(clk: ClockIdT) -> EResult<T> {
	let ts = current_time(clk, TimestampScale::Nanosecond)?;
	Ok(T::from_nano(ts))
}
