#![allow(unsafe_code)]
#![allow(warnings)]
#![allow(missing_docs)]
#![allow(unused_variables)]
#![no_main]
#![no_std]

#[rtic::app(device = stm32g4xx_hal::stm32g4::stm32g474, peripherals = true)]
mod app {
    use fugit::{
        Duration, 
        ExtU32, 
        HertzU32 as Hertz, 
        HoursDurationU32 as Hour,
        MicrosDurationU32 as MicroSecond, 
        MinutesDurationU32 as Minute, 
        NanosDurationU32 as NanoSecond,
        RateExtU32, 
        SecsDurationU32 as Second,
    };
    use lvgl::{
        Display,
        DrawBuffer,
    };
    use nb::block;
    use stm32g4xx_hal::delay::SYSTDelayExt;
    use stm32g4xx_hal::prelude::*;
    use stm32g4xx_hal::rcc::{PllConfig, RccExt};
    use stm32g4xx_hal::syscfg::SysCfgExt;
    use systick_monotonic::Systick;

    use panic_halt as _;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<1_000>;

    const SYSTEM_CLOCK_RATE: usize = 160_000_000;
    const SYSTICK_CLOCK_RATE: usize = 1_000_000;

    const MAJOR: u8 = 0;
    const MINOR: u8 = 1;
    const PATCH: u8 = 0;

    // Resources shared between tasks
    #[shared]
    struct Shared {
    }

    // Local resources to specific tasks (cannot be shared)
    #[local]
    struct Local {
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // let dp = Peripherals::take().unwrap();
        // let cp = cortex_m::Peripherals::take().expect("cannot take core peripherals");

        let dp = cx.device;
        let cp = cx.core;

        unsafe {
            // Enable DAC4
            dp.RCC.ahb2enr.modify(|r, w| w.dac4en().set_bit());
        }

        let rcc = dp.RCC.constrain();
        let mut pll_config = stm32g4xx_hal::rcc::PllConfig::default();

        // Sysclock is based on PLL_R
        pll_config.mux = stm32g4xx_hal::rcc::PLLSrc::HSI; // 16MHz
        pll_config.n = stm32g4xx_hal::rcc::PllNMul::MUL_20;
        pll_config.m = stm32g4xx_hal::rcc::PllMDiv::DIV_1; // f(vco) = 16MHz*20/2 = 320MHz
        pll_config.r = Some(stm32g4xx_hal::rcc::PllRDiv::DIV_2); // f(sysclock) = 320MHz/2 = 160MHz

        // Note to future self: The AHB clock runs the timers, among other things.
        // Please refer to the Clock Tree manual to determine if it is worth
        // changing to a lower speed for battery life savings.
        let mut clock_config = stm32g4xx_hal::rcc::Config::default()
            .pll_cfg(pll_config)
            .clock_src(stm32g4xx_hal::rcc::SysClockSrc::PLL);

        // After clock configuration, the following should be true:
        // Sysclock is 160MHz
        // AHB clock is 160MHz
        // APB1 clock is 160MHz
        // APB2 clock is 160MHz
        // The ADC will ultimately be put into synchronous mode and will derive
        // its clock from the AHB bus clock, with a prescalar of 2 or 4.

        let mut rcc = rcc.freeze(clock_config);
        let mut exti = dp.EXTI;
        let mut syscfg = dp.SYSCFG.constrain();

        // Configure Systick to be a 1ms timer
        let systick = cp.SYST;
        let mono = Systick::<1000>::new(systick, SYSTEM_CLOCK_RATE as u32);

        (
            // Initialization of shared resources
            Shared {
            },
            // Initialization of task local resources
            Local {
            },
            // Move the monotonic timer to the RTIC run-time, this enables
            // scheduling
            init::Monotonics(mono),
        )
    }

    // Background task, runs whenever no other tasks are running
    #[idle()]
    fn idle(mut cx: idle::Context) -> ! {
        let mut start = monotonics::now().ticks();

        const HOR_RES: u32 = 64;
        const VER_RES: u32 = 64;

        let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::default();

        let display = Display::register(buffer, HOR_RES, VER_RES, |refresh| {
            // sim_display.draw_iter(refresh.as_pixels()).unwrap();
        }).unwrap();

        loop {
            // Sleep until next interrupt
            cortex_m::asm::wfi();
        }
    }
}
