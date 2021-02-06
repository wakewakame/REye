// println!の代わりにlog!でコンソール出力できるようにする
#[allow(unused_macros)]
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
