mod step0;
mod step1;
mod step2;

#[cfg(feature = "step_0")]
pub fn run() { step0::main(); }

#[cfg(feature = "step_1")]
pub fn run() { step1::main(); }

#[cfg(feature = "step_2")]
pub fn run() { step2::main(); }
