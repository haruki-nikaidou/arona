#[cfg(feature = "amqprs")]
pub trait IntervalJobExecutionSignal: crate::amqp::AmqpMessageSend {
    fn tick(now: time::OffsetDateTime) -> Self;
    fn time_pool(
        now: time::OffsetDateTime,
        last_time: time::OffsetDateTime,
    ) -> std::task::Poll<Self>;
}