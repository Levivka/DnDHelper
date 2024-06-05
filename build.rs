use slint_build::CompilerConfiguration;

fn main() {
    slint_build::compile_with_config(
        "ui/activities/main.slint",
        CompilerConfiguration::new().with_style("fluent".to_string()),
    )
    .unwrap();

    // slint_build::compile_with_config(
    //     "ui/activities/profile.slint",
    //     CompilerConfiguration::new().with_style("fluent".to_string()),
    // ).unwrap();
}
