#[allow(dead_code)]
fn needs_sync<T: Sync>() {}

#[test]
fn event_loop_proxy_send() {
    #[allow(dead_code)]
    fn is_send<T: 'static + Send>() {
        // ensures that `rio_winit_fork::EventLoopProxy<T: Send>` implements `Sync`
        needs_sync::<rio_winit_fork::event_loop::EventLoopProxy<T>>();
    }
}

#[test]
fn window_sync() {
    // ensures that `rio_winit_fork::Window` implements `Sync`
    needs_sync::<rio_winit_fork::window::Window>();
}

#[test]
fn window_builder_sync() {
    needs_sync::<rio_winit_fork::window::WindowAttributes>();
}

#[test]
fn custom_cursor_sync() {
    needs_sync::<rio_winit_fork::window::CustomCursorSource>();
    needs_sync::<rio_winit_fork::window::CustomCursor>();
}
