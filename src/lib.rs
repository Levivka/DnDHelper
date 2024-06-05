use slint::SharedString;

slint::include_modules!();

#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak().unwrap();

    

    // ui.on_login_check(move |key, args| {
    //     let login_pass: SharedString = "sup".into();
    //     let password_pass: SharedString = "pass".into();

    //     if login == login_pass || password == password_pass {
    //         ui_handle.set_login_checked(true);
    //     }
    // });

    // ui.on_hello_world(move |login, password|{
    //     let msg_full = login + &password;
    //     ui_handle.set_full_text(msg_full);
    // });

    // ui.on_entry(move || {
    //     let check = true;
    //     ui_handle.set_login_checked(check);
    // });

    // ui.on_profile_select();

    ui.run().unwrap();
}
