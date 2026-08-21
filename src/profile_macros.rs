#[cfg(feature = "tracy")]
#[macro_export]
macro_rules! profile_zone {
    ($name:expr) => {
        let _tracy_span = tracy_client::span!($name, 0);
    };
}

#[cfg(not(feature = "tracy"))]
#[macro_export]
macro_rules! profile_zone {
    ($name:expr) => {};
}

#[cfg(feature = "tracy")]
#[macro_export]
macro_rules! profile_plot {
    ($name:expr, $var_name:ident, $val:expr) => {{
        let _ = stringify!($var_name);
        let client = tracy_client::Client::start();
        let plot_name = tracy_client::plot_name!($name);
        client.plot(plot_name, $val as f64);
    }};
}

#[cfg(not(feature = "tracy"))]
#[macro_export]
macro_rules! profile_plot {
    ($name:expr, $var_name:ident, $val:expr) => {};
}

#[cfg(feature = "tracy")]
#[macro_export]
macro_rules! set_thread_name {
    ($name:expr) => {
        tracy_client::Client::start().set_thread_name($name);
    };
}

#[cfg(not(feature = "tracy"))]
#[macro_export]
macro_rules! set_thread_name {
    ($name:expr) => {};
}

#[cfg(feature = "tracy")]
#[macro_export]
macro_rules! mark_frame {
    () => {
        tracy_client::Client::start().frame_mark();
    };
}

#[cfg(not(feature = "tracy"))]
#[macro_export]
macro_rules! mark_frame {
    () => {};
}
