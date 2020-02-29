macro_rules! define_step_module {
    ($module:ident, $feature:literal) => {
        mod $module;

        #[cfg(feature = $feature)]
        pub fn run() {
            $module::main();
        }
    };
}

define_step_module! { step0, "step_0" }
define_step_module! { step1, "step_1" }
define_step_module! { step2, "step_2" }
define_step_module! { step3, "step_3" }
define_step_module! { step4, "step_4" }
