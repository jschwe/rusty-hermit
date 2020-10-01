
/// Common functionality for all integration tests

extern crate test;
use test::{TestDescAndFn, TestFn, TestDesc};
//use std::borrow::Cow;
//use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
	Success = 0x10,
	Failed = 0x11,
}



pub fn test_runner(tests: &[& TestDescAndFn]) {
	println!("Running {} tests", tests.len());
	for test in tests {
		let test: Option<&fn()> = match &test.testfn {
			TestFn::StaticTestFn(a) => Some(a),
			_ => None,
		};
		let t = test.unwrap();
		t();

	}
	exit(false);
}

pub fn exit(failure: bool) -> ! {
	// temporarily make this public. FIXME: we could also pass an argument to main indicating uhyve or qemu
	unsafe {
		if hermit_abi::environment_is_uhyve().unwrap() {
			match failure {
				//ToDo: Add uhyve exit code enum
				true => hermit_abi::exit(1),
				false => hermit_abi::exit(0),
			}
		} else {
			match failure {
				true => exit_qemu(QemuExitCode::Failed),
				false => exit_qemu(QemuExitCode::Success),
			}
		}
	}
	
}

/// Debug exit from qemu with a returncode
/// '-device', 'isa-debug-exit,iobase=0xf4,iosize=0x04' must be passed to qemu for this to work
pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
	use x86::io::outl;

	unsafe {
		outl(0xf4, exit_code as u32);
	}
	println!("Warning - Failed to debug exit qemu - exiting via sys_exit()");
	unsafe { hermit_abi::exit(0) }//sys_exit exitcode on qemu gets silently dropped
}

//adapted from: https://rust-lang.github.io/rfcs/2360-bench-black-box.html
#[inline(always)]
pub fn value_fence<T>(x: T) -> T {
	let y = unsafe { (&x as *const T).read_volatile() };
	//std::hint::forget(x); - doesn't exist (anymore)
	y
}
