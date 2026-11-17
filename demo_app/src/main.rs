fn main() {
    native_kit::engine::dev_server::start::<demo_app::NativeApp>();
    std::thread::park();
}
