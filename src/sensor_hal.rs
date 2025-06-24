use crate::{marker::mode, Error, Tmp1x2};
#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;
#[cfg(feature = "embedded-sensors-hal")]
use embedded_sensors_hal::{sensor, temperature};
#[cfg(feature = "embedded-sensors-hal-async")]
use embedded_sensors_hal_async::{sensor, temperature};

#[cfg(all(
    feature = "embedded-sensors-hal",
    feature = "embedded-sensors-hal-async"
))]
compile_error!(
    "Only one of `embedded-sensors-hal` or `embedded-sensors-hal-async` must be enabled at a time."
);

#[cfg(any(
    feature = "embedded-sensors-hal",
    feature = "embedded-sensors-hal-async"
))]
impl<E: core::fmt::Debug> sensor::Error for Error<E> {
    fn kind(&self) -> sensor::ErrorKind {
        match *self {
            Error::I2C(_) => sensor::ErrorKind::Peripheral,
        }
    }
}

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "Tmp1x2",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<I2C, E, MODE> sensor::ErrorType for Tmp1x2<I2C, MODE>
where
    I2C: AsyncI2c<Error = E>,
    E: core::fmt::Debug,
{
    type Error = Error<E>;
}

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "Tmp1x2",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<I2C, E> temperature::TemperatureSensor for Tmp1x2<I2C, mode::Continuous>
where
    I2C: AsyncI2c<Error = E>,
    E: core::fmt::Debug,
{
    async fn temperature(&mut self) -> Result<temperature::DegreesCelsius, Error<E>> {
        self.read_temperature().await
    }
}

#[cfg(feature = "embedded-sensors-hal-async")]
impl<I2C, E> temperature::TemperatureThresholdSet for Tmp1x2<I2C, mode::Continuous>
where
    I2C: AsyncI2c<Error = E>,
    E: core::fmt::Debug,
{
    async fn set_temperature_threshold_low(
        &mut self,
        threshold: temperature::DegreesCelsius,
    ) -> Result<(), Error<E>> {
        self.set_low_temperature_threshold(threshold).await
    }

    async fn set_temperature_threshold_high(
        &mut self,
        threshold: temperature::DegreesCelsius,
    ) -> Result<(), Error<E>> {
        self.set_high_temperature_threshold(threshold).await
    }
}
