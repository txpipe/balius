use tracing_subscriber::{
    field::VisitOutput,
    fmt::format::{DefaultVisitor, Writer},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer, Registry,
};

#[derive(Default)]
pub struct BaliusLayer {}

impl BaliusLayer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S: tracing::Subscriber> Layer<S> for BaliusLayer {
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let context = event.metadata().target();
        let level = match *event.metadata().level() {
            tracing::Level::TRACE => crate::wit::balius::app::logging::Level::Trace,
            tracing::Level::DEBUG => crate::wit::balius::app::logging::Level::Debug,
            tracing::Level::INFO => crate::wit::balius::app::logging::Level::Info,
            tracing::Level::WARN => crate::wit::balius::app::logging::Level::Warn,
            tracing::Level::ERROR => crate::wit::balius::app::logging::Level::Error,
        };

        let mut message = String::new();
        let mut visitor = DefaultVisitor::new(Writer::new(&mut message), true);
        event.record(&mut visitor);
        if visitor.finish().is_err() {
            message = "[error formatting message]".to_string();
        }

        crate::wit::balius::app::logging::log(level, context, &message);
    }
}

pub fn init() {
    Registry::default().with(BaliusLayer::new()).init();
}
