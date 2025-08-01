#![no_std]
#![allow(async_fn_in_trait)]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

//! ## Feature flags
#![doc = document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#)]

// This mod MUST go first, so that the others see its macros.
pub(crate) mod fmt;

#[cfg(feature = "binary-info")]
pub use rp_binary_info as binary_info;

#[cfg(feature = "critical-section-impl")]
mod critical_section_impl;

#[cfg(feature = "rp2040")]
mod intrinsics;

pub mod adc;
#[cfg(feature = "_rp235x")]
pub mod block;
#[cfg(feature = "rp2040")]
pub mod bootsel;
pub mod clocks;
pub mod dma;
pub mod flash;
#[cfg(feature = "rp2040")]
mod float;
pub mod gpio;
pub mod i2c;
pub mod i2c_slave;
pub mod multicore;
#[cfg(feature = "_rp235x")]
pub mod otp;
pub mod pio_programs;
pub mod pwm;
mod reset;
pub mod rom_data;
#[cfg(feature = "rp2040")]
pub mod rtc;
pub mod spi;
mod spinlock;
pub mod spinlock_mutex;
#[cfg(feature = "time-driver")]
pub mod time_driver;
#[cfg(feature = "_rp235x")]
pub mod trng;
pub mod uart;
pub mod usb;
pub mod watchdog;

// PIO
pub mod pio;
pub(crate) mod relocate;

// Reexports
pub use embassy_hal_internal::{Peri, PeripheralType};
#[cfg(feature = "unstable-pac")]
pub use rp_pac as pac;
#[cfg(not(feature = "unstable-pac"))]
pub(crate) use rp_pac as pac;

#[cfg(feature = "rt")]
pub use crate::pac::NVIC_PRIO_BITS;

#[cfg(feature = "rp2040")]
embassy_hal_internal::interrupt_mod!(
    TIMER_IRQ_0,
    TIMER_IRQ_1,
    TIMER_IRQ_2,
    TIMER_IRQ_3,
    PWM_IRQ_WRAP,
    USBCTRL_IRQ,
    XIP_IRQ,
    PIO0_IRQ_0,
    PIO0_IRQ_1,
    PIO1_IRQ_0,
    PIO1_IRQ_1,
    DMA_IRQ_0,
    DMA_IRQ_1,
    IO_IRQ_BANK0,
    IO_IRQ_QSPI,
    SIO_IRQ_PROC0,
    SIO_IRQ_PROC1,
    CLOCKS_IRQ,
    SPI0_IRQ,
    SPI1_IRQ,
    UART0_IRQ,
    UART1_IRQ,
    ADC_IRQ_FIFO,
    I2C0_IRQ,
    I2C1_IRQ,
    RTC_IRQ,
    SWI_IRQ_0,
    SWI_IRQ_1,
    SWI_IRQ_2,
    SWI_IRQ_3,
    SWI_IRQ_4,
    SWI_IRQ_5,
);

#[cfg(feature = "_rp235x")]
embassy_hal_internal::interrupt_mod!(
    TIMER0_IRQ_0,
    TIMER0_IRQ_1,
    TIMER0_IRQ_2,
    TIMER0_IRQ_3,
    TIMER1_IRQ_0,
    TIMER1_IRQ_1,
    TIMER1_IRQ_2,
    TIMER1_IRQ_3,
    PWM_IRQ_WRAP_0,
    PWM_IRQ_WRAP_1,
    DMA_IRQ_0,
    DMA_IRQ_1,
    USBCTRL_IRQ,
    PIO0_IRQ_0,
    PIO0_IRQ_1,
    PIO1_IRQ_0,
    PIO1_IRQ_1,
    PIO2_IRQ_0,
    PIO2_IRQ_1,
    IO_IRQ_BANK0,
    IO_IRQ_BANK0_NS,
    IO_IRQ_QSPI,
    IO_IRQ_QSPI_NS,
    SIO_IRQ_FIFO,
    SIO_IRQ_BELL,
    SIO_IRQ_FIFO_NS,
    SIO_IRQ_BELL_NS,
    CLOCKS_IRQ,
    SPI0_IRQ,
    SPI1_IRQ,
    UART0_IRQ,
    UART1_IRQ,
    ADC_IRQ_FIFO,
    I2C0_IRQ,
    I2C1_IRQ,
    TRNG_IRQ,
    PLL_SYS_IRQ,
    PLL_USB_IRQ,
    SWI_IRQ_0,
    SWI_IRQ_1,
    SWI_IRQ_2,
    SWI_IRQ_3,
    SWI_IRQ_4,
    SWI_IRQ_5,
);

/// Macro to bind interrupts to handlers.
///
/// This defines the right interrupt handlers, and creates a unit struct (like `struct Irqs;`)
/// and implements the right [`Binding`]s for it. You can pass this struct to drivers to
/// prove at compile-time that the right interrupts have been bound.
///
/// Example of how to bind one interrupt:
///
/// ```rust,ignore
/// use embassy_rp::{bind_interrupts, usb, peripherals};
///
/// bind_interrupts!(
///     /// Binds the USB Interrupts.
///     struct Irqs {
///         USBCTRL_IRQ => usb::InterruptHandler<peripherals::USB>;
///     }
/// );
/// ```
///
// developer note: this macro can't be in `embassy-hal-internal` due to the use of `$crate`.
#[macro_export]
macro_rules! bind_interrupts {
    ($(#[$attr:meta])* $vis:vis struct $name:ident {
        $(
            $(#[cfg($cond_irq:meta)])?
            $irq:ident => $(
                $(#[cfg($cond_handler:meta)])?
                $handler:ty
            ),*;
        )*
    }) => {
        #[derive(Copy, Clone)]
        $(#[$attr])*
        $vis struct $name;

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            $(#[cfg($cond_irq)])?
            unsafe extern "C" fn $irq() {
                unsafe {
                    $(
                        $(#[cfg($cond_handler)])?
                        <$handler as $crate::interrupt::typelevel::Handler<$crate::interrupt::typelevel::$irq>>::on_interrupt();

                    )*
                }
            }

            $(#[cfg($cond_irq)])?
            $crate::bind_interrupts!(@inner
                $(
                    $(#[cfg($cond_handler)])?
                    unsafe impl $crate::interrupt::typelevel::Binding<$crate::interrupt::typelevel::$irq, $handler> for $name {}
                )*
            );
        )*
    };
    (@inner $($t:tt)*) => {
        $($t)*
    }
}

#[cfg(feature = "rp2040")]
embassy_hal_internal::peripherals! {
    PIN_0,
    PIN_1,
    PIN_2,
    PIN_3,
    PIN_4,
    PIN_5,
    PIN_6,
    PIN_7,
    PIN_8,
    PIN_9,
    PIN_10,
    PIN_11,
    PIN_12,
    PIN_13,
    PIN_14,
    PIN_15,
    PIN_16,
    PIN_17,
    PIN_18,
    PIN_19,
    PIN_20,
    PIN_21,
    PIN_22,
    PIN_23,
    PIN_24,
    PIN_25,
    PIN_26,
    PIN_27,
    PIN_28,
    PIN_29,
    PIN_QSPI_SCLK,
    PIN_QSPI_SS,
    PIN_QSPI_SD0,
    PIN_QSPI_SD1,
    PIN_QSPI_SD2,
    PIN_QSPI_SD3,

    UART0,
    UART1,

    SPI0,
    SPI1,

    I2C0,
    I2C1,

    DMA_CH0,
    DMA_CH1,
    DMA_CH2,
    DMA_CH3,
    DMA_CH4,
    DMA_CH5,
    DMA_CH6,
    DMA_CH7,
    DMA_CH8,
    DMA_CH9,
    DMA_CH10,
    DMA_CH11,

    PWM_SLICE0,
    PWM_SLICE1,
    PWM_SLICE2,
    PWM_SLICE3,
    PWM_SLICE4,
    PWM_SLICE5,
    PWM_SLICE6,
    PWM_SLICE7,

    USB,

    RTC,

    FLASH,

    ADC,
    ADC_TEMP_SENSOR,

    CORE1,

    PIO0,
    PIO1,

    WATCHDOG,
    BOOTSEL,
}

#[cfg(feature = "_rp235x")]
embassy_hal_internal::peripherals! {
    PIN_0,
    PIN_1,
    PIN_2,
    PIN_3,
    PIN_4,
    PIN_5,
    PIN_6,
    PIN_7,
    PIN_8,
    PIN_9,
    PIN_10,
    PIN_11,
    PIN_12,
    PIN_13,
    PIN_14,
    PIN_15,
    PIN_16,
    PIN_17,
    PIN_18,
    PIN_19,
    PIN_20,
    PIN_21,
    PIN_22,
    PIN_23,
    PIN_24,
    PIN_25,
    PIN_26,
    PIN_27,
    PIN_28,
    PIN_29,
    #[cfg(feature = "rp235xb")]
    PIN_30,
    #[cfg(feature = "rp235xb")]
    PIN_31,
    #[cfg(feature = "rp235xb")]
    PIN_32,
    #[cfg(feature = "rp235xb")]
    PIN_33,
    #[cfg(feature = "rp235xb")]
    PIN_34,
    #[cfg(feature = "rp235xb")]
    PIN_35,
    #[cfg(feature = "rp235xb")]
    PIN_36,
    #[cfg(feature = "rp235xb")]
    PIN_37,
    #[cfg(feature = "rp235xb")]
    PIN_38,
    #[cfg(feature = "rp235xb")]
    PIN_39,
    #[cfg(feature = "rp235xb")]
    PIN_40,
    #[cfg(feature = "rp235xb")]
    PIN_41,
    #[cfg(feature = "rp235xb")]
    PIN_42,
    #[cfg(feature = "rp235xb")]
    PIN_43,
    #[cfg(feature = "rp235xb")]
    PIN_44,
    #[cfg(feature = "rp235xb")]
    PIN_45,
    #[cfg(feature = "rp235xb")]
    PIN_46,
    #[cfg(feature = "rp235xb")]
    PIN_47,
    PIN_QSPI_SCLK,
    PIN_QSPI_SS,
    PIN_QSPI_SD0,
    PIN_QSPI_SD1,
    PIN_QSPI_SD2,
    PIN_QSPI_SD3,

    UART0,
    UART1,

    SPI0,
    SPI1,

    I2C0,
    I2C1,

    DMA_CH0,
    DMA_CH1,
    DMA_CH2,
    DMA_CH3,
    DMA_CH4,
    DMA_CH5,
    DMA_CH6,
    DMA_CH7,
    DMA_CH8,
    DMA_CH9,
    DMA_CH10,
    DMA_CH11,
    DMA_CH12,
    DMA_CH13,
    DMA_CH14,
    DMA_CH15,

    PWM_SLICE0,
    PWM_SLICE1,
    PWM_SLICE2,
    PWM_SLICE3,
    PWM_SLICE4,
    PWM_SLICE5,
    PWM_SLICE6,
    PWM_SLICE7,
    PWM_SLICE8,
    PWM_SLICE9,
    PWM_SLICE10,
    PWM_SLICE11,

    USB,

    RTC,

    FLASH,

    ADC,
    ADC_TEMP_SENSOR,

    CORE1,

    PIO0,
    PIO1,
    PIO2,

    WATCHDOG,
    BOOTSEL,

    TRNG
}

#[cfg(all(not(feature = "boot2-none"), feature = "rp2040"))]
macro_rules! select_bootloader {
    ( $( $feature:literal => $loader:ident, )+ default => $default:ident ) => {
        $(
            #[cfg(feature = $feature)]
            #[link_section = ".boot2"]
            #[used]
            static BOOT2: [u8; 256] = rp2040_boot2::$loader;
        )*

        #[cfg(not(any( $( feature = $feature),* )))]
        #[link_section = ".boot2"]
        #[used]
        static BOOT2: [u8; 256] = rp2040_boot2::$default;
    }
}

#[cfg(all(not(feature = "boot2-none"), feature = "rp2040"))]
select_bootloader! {
    "boot2-at25sf128a" => BOOT_LOADER_AT25SF128A,
    "boot2-gd25q64cs" => BOOT_LOADER_GD25Q64CS,
    "boot2-generic-03h" => BOOT_LOADER_GENERIC_03H,
    "boot2-is25lp080" => BOOT_LOADER_IS25LP080,
    "boot2-ram-memcpy" => BOOT_LOADER_RAM_MEMCPY,
    "boot2-w25q080" => BOOT_LOADER_W25Q080,
    "boot2-w25x10cl" => BOOT_LOADER_W25X10CL,
    default => BOOT_LOADER_W25Q080
}

#[cfg(all(not(feature = "imagedef-none"), feature = "_rp235x"))]
macro_rules! select_imagedef {
    ( $( $feature:literal => $imagedef:ident, )+ default => $default:ident ) => {
        $(
            #[cfg(feature = $feature)]
            #[link_section = ".start_block"]
            #[used]
            static IMAGE_DEF: crate::block::ImageDef = crate::block::ImageDef::$imagedef();
        )*

        #[cfg(not(any( $( feature = $feature),* )))]
        #[link_section = ".start_block"]
        #[used]
        static IMAGE_DEF: crate::block::ImageDef = crate::block::ImageDef::$default();
    }
}

#[cfg(all(not(feature = "imagedef-none"), feature = "_rp235x"))]
select_imagedef! {
    "imagedef-secure-exe" => secure_exe,
    "imagedef-nonsecure-exe" => non_secure_exe,
    default => secure_exe
}

/// Installs a stack guard for the CORE0 stack in MPU region 0.
/// Will fail if the MPU is already configured. This function requires
/// a `_stack_end` symbol to be defined by the linker script, and expects
/// `_stack_end` to be located at the lowest address (largest depth) of
/// the stack.
///
/// This method can *only* set up stack guards on the currently
/// executing core. Stack guards for CORE1 are set up automatically,
/// only CORE0 should ever use this.
///
/// # Usage
///
/// ```no_run
/// use embassy_rp::install_core0_stack_guard;
/// use embassy_executor::{Executor, Spawner};
///
/// #[embassy_executor::main]
/// async fn main(_spawner: Spawner) {
///     // set up by the linker as follows:
///     //
///     //     MEMORY {
///     //       STACK0: ORIGIN = 0x20040000, LENGTH = 4K
///     //     }
///     //
///     //     _stack_end = ORIGIN(STACK0);
///     //     _stack_start = _stack_end + LENGTH(STACK0);
///     //
///     install_core0_stack_guard().expect("MPU already configured");
///     let p = embassy_rp::init(Default::default());
///
///     // ...
/// }
/// ```
pub fn install_core0_stack_guard() -> Result<(), ()> {
    extern "C" {
        static mut _stack_end: usize;
    }
    unsafe { install_stack_guard(core::ptr::addr_of_mut!(_stack_end)) }
}

#[cfg(all(feature = "rp2040", not(feature = "_test")))]
#[inline(always)]
unsafe fn install_stack_guard(stack_bottom: *mut usize) -> Result<(), ()> {
    let core = unsafe { cortex_m::Peripherals::steal() };

    // Fail if MPU is already configured
    if core.MPU.ctrl.read() != 0 {
        return Err(());
    }

    // The minimum we can protect is 32 bytes on a 32 byte boundary, so round up which will
    // just shorten the valid stack range a tad.
    let addr = (stack_bottom as u32 + 31) & !31;
    // Mask is 1 bit per 32 bytes of the 256 byte range... clear the bit for the segment we want
    let subregion_select = 0xff ^ (1 << ((addr >> 5) & 7));
    unsafe {
        core.MPU.ctrl.write(5); // enable mpu with background default map
        core.MPU.rbar.write((addr & !0xff) | (1 << 4)); // set address and update RNR
        core.MPU.rasr.write(
            1 // enable region
               | (0x7 << 1) // size 2^(7 + 1) = 256
               | (subregion_select << 8)
               | 0x10000000, // XN = disable instruction fetch; no other bits means no permissions
        );
    }
    Ok(())
}

#[cfg(all(feature = "_rp235x", not(feature = "_test")))]
#[inline(always)]
unsafe fn install_stack_guard(stack_bottom: *mut usize) -> Result<(), ()> {
    let core = unsafe { cortex_m::Peripherals::steal() };

    // Fail if MPU is already configured
    if core.MPU.ctrl.read() != 0 {
        return Err(());
    }

    unsafe {
        core.MPU.ctrl.write(5); // enable mpu with background default map
        core.MPU.rbar.write(stack_bottom as u32 & !0xff); // set address
        core.MPU.rlar.write(((stack_bottom as usize + 255) as u32) | 1);
    }
    Ok(())
}

// This is to hack around cortex_m defaulting to ARMv7 when building tests,
// so the compile fails when we try to use ARMv8 peripherals.
#[cfg(feature = "_test")]
#[inline(always)]
unsafe fn install_stack_guard(_stack_bottom: *mut usize) -> Result<(), ()> {
    Ok(())
}

/// HAL configuration for RP.
pub mod config {
    use crate::clocks::ClockConfig;

    /// HAL configuration passed when initializing.
    #[non_exhaustive]
    pub struct Config {
        /// Clock configuration.
        pub clocks: ClockConfig,
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                clocks: ClockConfig::crystal(12_000_000),
            }
        }
    }

    impl Config {
        /// Create a new configuration with the provided clock config.
        pub fn new(clocks: ClockConfig) -> Self {
            Self { clocks }
        }
    }
}

/// Initialize the `embassy-rp` HAL with the provided configuration.
///
/// This returns the peripheral singletons that can be used for creating drivers.
///
/// This should only be called once at startup, otherwise it panics.
pub fn init(config: config::Config) -> Peripherals {
    // Do this first, so that it panics if user is calling `init` a second time
    // before doing anything important.
    let peripherals = Peripherals::take();

    unsafe {
        clocks::init(config.clocks);
        #[cfg(feature = "time-driver")]
        time_driver::init();
        dma::init();
        gpio::init();
    }

    peripherals
}

#[cfg(feature = "rt")]
#[cortex_m_rt::pre_init]
unsafe fn pre_init() {
    // SIO does not get reset when core0 is reset with either `scb::sys_reset()` or with SWD.
    // Since we're using SIO spinlock 31 for the critical-section impl, this causes random
    // hangs if we reset in the middle of a CS, because the next boot sees the spinlock
    // as locked and waits forever.
    //
    // See https://github.com/embassy-rs/embassy/issues/1736
    // and https://github.com/rp-rs/rp-hal/issues/292
    // and https://matrix.to/#/!vhKMWjizPZBgKeknOo:matrix.org/$VfOkQgyf1PjmaXZbtycFzrCje1RorAXd8BQFHTl4d5M
    //
    // According to Raspberry Pi, this is considered Working As Intended, and not an errata,
    // even though this behavior is different from every other ARM chip (sys_reset usually resets
    // the *system* as its name implies, not just the current core).
    //
    // To fix this, reset SIO on boot. We must do this in pre_init because it's unsound to do it
    // in `embassy_rp::init`, since the user could've acquired a CS by then. pre_init is guaranteed
    // to run before any user code.
    //
    // A similar thing could happen with PROC1. It is unclear whether it's possible for PROC1
    // to stay unreset through a PROC0 reset, so we reset it anyway just in case.
    //
    // Important info from PSM logic (from Luke Wren in above Matrix thread)
    //
    //     The logic is, each PSM stage is reset if either of the following is true:
    //     - The previous stage is in reset and FRCE_ON is false
    //     - FRCE_OFF is true
    //
    // The PSM order is SIO -> PROC0 -> PROC1.
    // So, we have to force-on PROC0 to prevent it from getting reset when resetting SIO.
    #[cfg(feature = "rp2040")]
    {
        pac::PSM.frce_on().write_and_wait(|w| {
            w.set_proc0(true);
        });
        // Then reset SIO and PROC1.
        pac::PSM.frce_off().write_and_wait(|w| {
            w.set_sio(true);
            w.set_proc1(true);
        });
        // clear force_off first, force_on second. The other way around would reset PROC0.
        pac::PSM.frce_off().write_and_wait(|_| {});
        pac::PSM.frce_on().write_and_wait(|_| {});
    }

    #[cfg(feature = "_rp235x")]
    {
        // on RP235x, datasheet says "The FRCE_ON register is a development feature that does nothing in production devices."
        // No idea why they removed it. Removing it means we can't use PSM to reset SIO, because it comes before
        // PROC0, so we'd need FRCE_ON to prevent resetting ourselves.
        //
        // So we just unlock the spinlock manually.
        pac::SIO.spinlock(31).write_value(1);

        // We can still use PSM to reset PROC1 since it comes after PROC0 in the state machine.
        pac::PSM.frce_off().write_and_wait(|w| w.set_proc1(true));
        pac::PSM.frce_off().write_and_wait(|_| {});

        // Make atomics work between cores.
        enable_actlr_extexclall();
    }
}

/// Set the EXTEXCLALL bit in ACTLR.
///
/// The default MPU memory map marks all memory as non-shareable, so atomics don't
/// synchronize memory accesses between cores at all. This bit forces all memory to be
/// considered shareable regardless of what the MPU says.
///
/// TODO: does this interfere somehow if the user wants to use a custom MPU configuration?
/// maybe we need to add a way to disable this?
///
/// This must be done FOR EACH CORE.
#[cfg(feature = "_rp235x")]
unsafe fn enable_actlr_extexclall() {
    (&*cortex_m::peripheral::ICB::PTR).actlr.modify(|w| w | (1 << 29));
}

/// Extension trait for PAC regs, adding atomic xor/bitset/bitclear writes.
#[allow(unused)]
trait RegExt<T: Copy> {
    #[allow(unused)]
    fn write_xor<R>(&self, f: impl FnOnce(&mut T) -> R) -> R;
    fn write_set<R>(&self, f: impl FnOnce(&mut T) -> R) -> R;
    fn write_clear<R>(&self, f: impl FnOnce(&mut T) -> R) -> R;
    fn write_and_wait<R>(&self, f: impl FnOnce(&mut T) -> R) -> R
    where
        T: PartialEq;
}

impl<T: Default + Copy, A: pac::common::Write> RegExt<T> for pac::common::Reg<T, A> {
    fn write_xor<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut val = Default::default();
        let res = f(&mut val);
        unsafe {
            let ptr = (self.as_ptr() as *mut u8).add(0x1000) as *mut T;
            ptr.write_volatile(val);
        }
        res
    }

    fn write_set<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut val = Default::default();
        let res = f(&mut val);
        unsafe {
            let ptr = (self.as_ptr() as *mut u8).add(0x2000) as *mut T;
            ptr.write_volatile(val);
        }
        res
    }

    fn write_clear<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut val = Default::default();
        let res = f(&mut val);
        unsafe {
            let ptr = (self.as_ptr() as *mut u8).add(0x3000) as *mut T;
            ptr.write_volatile(val);
        }
        res
    }

    fn write_and_wait<R>(&self, f: impl FnOnce(&mut T) -> R) -> R
    where
        T: PartialEq,
    {
        let mut val = Default::default();
        let res = f(&mut val);
        unsafe {
            self.as_ptr().write_volatile(val);
            while self.as_ptr().read_volatile() != val {}
        }
        res
    }
}
