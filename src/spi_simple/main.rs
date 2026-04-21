//! Simple spi
#![no_std]
#![no_main]

use teensy4_panic as _;

pub mod controller;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [KPP])]
mod app {

    use bsp::board;
    use embedded_hal::spi::{Mode, Phase, Polarity};
    use imxrt_log as logging;
    use teensy4_bsp::{
        self as bsp,
        board::lpspi,
        hal::{
            gpio::Output,
            lpspi::{SamplePoint, Status},
        },
    };

    use rtic_monotonics::systick::*;

    use crate::controller::{spi_data_address, RegisterAddress};

    #[local]
    struct Local {
        /// For driving the logging endpoint.
        poller: logging::Poller,

        spi: board::Lpspi4,
        step_pin: Output<bsp::pins::t40::P9>,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            usb,
            pins,
            lpspi4, //
            mut gpio2,
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
            1_000_000,
        );

        spi.set_bit_order(teensy4_bsp::hal::lpspi::BitOrder::Msb);
        spi.disabled(|spi| {
            spi.set_sample_point(SamplePoint::Edge);
        });
        spi.set_mode(Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnSecondTransition,
        });

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

        let step_pin = gpio2.output(pins.p9);
        step_pin.clear();
        // Schedule that software task to run.
        spi_log_registers::spawn().unwrap();
        // stepper::spawn().unwrap();
        (
            Shared {},
            Local {
                poller,
                spi,
                step_pin,
            },
        )
    }

    /// this steps the stepper motor
    #[task(local = [step_pin])]
    async fn stepper(cx: stepper::Context) {
        let stepper::LocalResources { step_pin, .. } = cx.local;
        Systick::delay(7500.millis()).await;
        // for right now, it's fine to just hit the step_pin
        loop {
            step_pin.toggle();
            Systick::delay(1.millis()).await;
            step_pin.toggle();
            Systick::delay(1.millis()).await;
        }
    }

    fn result<E: core::fmt::Debug>(r: Result<&[u16], E>, prepend: &str) {
        match r {
            Ok(rx) => {
                log::info!("{prepend}:\t{:#018b}", rx[0]);
            }
            Err(e) => {
                log::error!("{:?}", e);
            }
        }
    }
    #[task(local = [spi])]
    async fn spi_log_registers(cx: spi_log_registers::Context) {
        let spi_log_registers::LocalResources { spi, .. } = cx.local;
        use embedded_hal::blocking::spi::Transfer;

        let tx_data_writing = [spi_data_address(
            0b01110011,
            RegisterAddress::Ctrl3 as u8,
            false,
        )];
        let tx_data_reading = [
            // Reads
            spi_data_address(0x00, RegisterAddress::Fault as u8, true), // 0x00
            spi_data_address(0x00, RegisterAddress::Diag1 as u8, true), // 0x01
            spi_data_address(0x00, RegisterAddress::Diag2 as u8, true), // 0x02
            spi_data_address(0x00, RegisterAddress::Ctrl1 as u8, true), // 0x03
            spi_data_address(0x00, RegisterAddress::Ctrl2 as u8, true), // 0x04
            spi_data_address(0x00, RegisterAddress::Ctrl3 as u8, true), // 0x05
            spi_data_address(0x00, RegisterAddress::Ctrl4 as u8, true), // 0x06
            spi_data_address(0x00, RegisterAddress::Ctrl5 as u8, true), // 0x07
            spi_data_address(0x00, RegisterAddress::Ctrl6 as u8, true), // 0x08
            spi_data_address(0x00, RegisterAddress::Ctrl7 as u8, true), // 0x09
            spi_data_address(0x00, RegisterAddress::Ctrl8 as u8, true), // 0x0A
            spi_data_address(0x00, RegisterAddress::Ctrl9 as u8, true), // 0x0B
        ];
        let tx_data_write_once = [spi_data_address(
            0b10001111,
            RegisterAddress::Ctrl2 as u8,
            false,
        )];

        // Systick::delay(7500.millis()).await;
        log::info!("STARTING");

        log::info!(
    "tx_data: {:#018b}, {:#018b}, {:#018b}, {:#018b}, {:#018b}, {:#018b}, {:#018b}, {:#018b}, {:#018b}, {:#018b}, {:#018b}, {:#018b}",
    tx_data_reading[0],
    tx_data_reading[1],
    tx_data_reading[2],
    tx_data_reading[3],
    tx_data_reading[4],
    tx_data_reading[5],
    tx_data_reading[6],
    tx_data_reading[7],
    tx_data_reading[8],
    tx_data_reading[9],
    tx_data_reading[10],
    tx_data_reading[11],
);
        /*
        Basic data I got         Status   Report
        0x00: Fault : WORD: 0b11 000000 00000000
        0x01: Diag1 : WORD: 0b11 000000 00000000
        0x02: Diag2 : WORD: 0b11 000000 00000000
        0x03: Ctrl1 : WORD: 0b11 000000 00000000
        0x04: Ctrl2 : WORD: 0b11 000000 00001111
        0x05: Ctrl3 : WORD: 0b11 000000 00000110
        0x06: Ctrl4 : WORD: 0b11 000000 00110000
        0x07: Ctrl5 : WORD: 0b11 000000 00001000
        0x08: Ctrl6 : WORD: 0b11 000000 00000011
        0x09: Ctrl7 : WORD: 0b11 000000 00100000
        0x0A: Ctrl8 : WORD: 0b11 000000 11111111
        0x0B: Ctrl9 : WORD: 0b11 000000 00001111


                */
        log::info!("STATUS: {:?}", spi.status());

        for tx_write_once in tx_data_write_once {
            result(spi.transfer(&mut [tx_write_once]), "WRITE");
        }

        loop {
            spi.clear_status(Status::all());
            // drv8434S requires csc to reset in between transfers
            // so we can't bulk transfer
            for tx_read in tx_data_reading {
                result(spi.transfer(&mut [tx_read]), "READ");
            }
            for tx_write in tx_data_writing {
                // drv8434S requires csc to reset in between transfers
                // so we can't bulk transfer
                result(spi.transfer(&mut [tx_write]), "WRITE");
            }

            log::info!("STATUS: {:?}", spi.status());
            // log::info!("FIFO STATUS:{:?}", spi.fifo_status());
            // spi.clear_fifos();
            Systick::delay(1.millis()).await;
        }
    }

    /// This task runs when the USB1 interrupt activates.
    /// Simply poll the logger to control the logging process.
    #[task(binds = USB_OTG1, local = [poller])]
    fn usb_interrupt(cx: usb_interrupt::Context) {
        cx.local.poller.poll();
    }
}
