//! Simple spi for LIS3DH accelerometer
#![no_std]
#![no_main]

use teensy4_panic as _;
mod accel;
#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [KPP])]
mod app {

    use bsp::board;
    use embedded_hal::spi::{Mode, Phase, Polarity};
    use imxrt_log as logging;
    use teensy4_bsp::{
        self as bsp,
        board::lpspi,
        hal::lpspi::{SamplePoint, Status},
    };

    use rtic_monotonics::systick::*;

    use crate::accel::{read_x_float, read_y_float, read_z_float, write_register, RegisterAddress};

    #[local]
    struct Local {
        /// For driving the logging endpoint.
        poller: logging::Poller,

        spi: board::Lpspi4,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            usb,
            pins,
            lpspi4, //
            ..
        } = board::t40(cx.device);

        let mut spi = lpspi(
            lpspi4,
            board::LpspiPins {
                pcs0: pins.p10,
                sdo: pins.p11,
                sdi: pins.p12,
                sck: pins.p13,
            },
            10_000_000,
        );

        spi.set_bit_order(teensy4_bsp::hal::lpspi::BitOrder::Msb);

        // Set up the logging system.
        //
        // There's various ways to control log levels at build- and run-time.
        // See the imxrt-log documentation for more information. This example
        // doesn't demonstrate any of that.
        let poller = logging::log::usbd(usb, logging::Interrupts::Enabled).unwrap();

        // Set up a system timer for our software task.
        {
            Systick::start(
                cx.core.SYST,
                board::ARM_FREQUENCY,
                rtic_monotonics::create_systick_token!(),
            );
        }

        // Schedule that software task to run.
        spi_log_registers::spawn().unwrap();
        (Shared {}, Local { poller, spi })
    }

    /// f32::NAN on error
    fn result<E: core::fmt::Debug>(r: Result<f32, E>) -> f32 {
        match r {
            Ok(rx) => {
                // log::info!("{:.02e}", rx);
                rx
            }
            Err(e) => {
                log::error!("{:?}", e);
                f32::NAN
            }
        }
    }

    #[task(local = [spi])]
    async fn spi_log_registers(cx: spi_log_registers::Context) {
        let spi_log_registers::LocalResources { spi, .. } = cx.local;

        // Systick::delay(7500.millis()).await;
        log::info!("STARTING");

        // write odr 1001 for 5khz normal, plus no lp and enable all axes
        if let Err(e) = write_register(spi, RegisterAddress::CtrlReg1, 0b10010111) {
            log::error!("{:?}", e);
        }

        loop {
            let x = result(read_x_float(spi, 2));
            let y = result(read_y_float(spi, 2));
            let z = result(read_z_float(spi, 2));

            log::info!("(x,y,z): ({x},{y},{z})");

            Systick::delay(100.millis()).await;
        }
    }

    /// This task runs when the USB1 interrupt activates.
    /// Simply poll the logger to control the logging process.
    #[task(binds = USB_OTG1, local = [poller])]
    fn usb_interrupt(cx: usb_interrupt::Context) {
        cx.local.poller.poll();
    }
}
