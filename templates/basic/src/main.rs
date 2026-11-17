fn main() {
    native_kit::engine::dev_server::start::<__CRATE_IDENT__::NativeApp>();
    std::thread::park();
}
