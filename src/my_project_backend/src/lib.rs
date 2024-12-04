use std::cell::RefCell;

thread_local! {
    static MSG: RefCell<String> = RefCell::new(String::new());
}

#[ic_cdk::update]
fn set_msg(message: String) {
    MSG.with(|msg| {
        *msg.borrow_mut() = message
    })
}

#[ic_cdk::query]
fn get_message() -> String {
    MSG.with(|msg| msg.borrow().clone())
}
