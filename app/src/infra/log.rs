use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init() {
    let fmt = tracing_subscriber::fmt::format().without_time().compact();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::builder().parse_lossy("error,app=info,api=info"))
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(
                    std::io::stdout
                        .with_min_level(tracing::Level::INFO)
                        .with_max_level(tracing::Level::INFO),
                )
                .event_format(fmt.clone())
                .with_ansi(atty::is(atty::Stream::Stdout)),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(
                    std::io::stderr
                        .with_min_level(tracing::Level::ERROR)
                        .with_max_level(tracing::Level::WARN),
                )
                .event_format(fmt)
                .with_ansi(atty::is(atty::Stream::Stderr)),
        )
        .init();
}
