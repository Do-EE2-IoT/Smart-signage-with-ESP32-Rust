// Cố xử lý nốt lịch học 2 tuần nữa
// 14/1/2025 - FPGA
// 15/1/2025 - M1
// 16/1/2025 - IoT
// Maybe thứ 6, hoàn thành đatn nhanh nhất có thể

//export LIBCLANG_PATH="/home/do30032003/.rustup/toolchains/esp/xtensa-esp32-elf-clang/esp-18.1.2_20240912/esp-clang/lib"
#![no_std]
#![no_main]
use asset::big::bien_90;
use bleps::{
    ad_structure::{
        create_advertising_data, AdStructure, BR_EDR_NOT_SUPPORTED, LE_GENERAL_DISCOVERABLE,
    },
    attribute_server::{AttributeServer, NotificationData, WorkResult},
    event::EventType,
    gatt, Ble, HciConnector, PollResult,
};
use core::cell::RefCell;
use display_interface_spi::SPIInterface;
use embassy_executor::Spawner;
use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::{
        ascii::{FONT_10X20, FONT_7X13},
        MonoTextStyle,
    },
    pixelcolor::{raw::BigEndian, Rgb565},
    prelude::*,
    primitives::{Arc, Circle, PrimitiveStyleBuilder},
    text::Text,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_alloc as _;
use esp_alloc as _;
use esp_backtrace as _;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{self, Input, Io, Level, NoPin, Output, Pull},
    prelude::*,
    rng::Rng,
    spi::{master::Spi, SpiMode},
    time,
    timer::timg::TimerGroup,
};
use esp_println::println;
use esp_wifi::ble::controller::BleConnector;
use esp_wifi::EspWifiInitFor;
use mipidsi::{
    models::GC9A01,
    options::{ColorInversion, ColorOrder},
    Builder,
};
pub mod asset;

fn get_cell_position(row: usize, col: usize, cell_size: usize) -> Point {
    let x = (col * cell_size) as i32;
    let y = (row * cell_size) as i32;
    Point::new(x, y)
}
#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) -> ! {
    let connected = RefCell::new(false);
    let bien_90 = RefCell::new(false);

    esp_println::logger::init_logger_from_env();
    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(72 * 1024);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let init = esp_wifi::init(
        EspWifiInitFor::Ble,
        timg0.timer0,
        Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();
    esp_hal_embassy::init(timg0.timer1);
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let button = Input::new(io.pins.gpio19, Pull::Up);
    let mut debounce_cnt = 500;
    let mut delay = Delay::new();

    // LCD Code

    let dc = Output::new(io.pins.gpio27, Level::High);
    let sck = io.pins.gpio14;
    let miso = io.pins.gpio12;
    let mosi = io.pins.gpio15;
    let cs = io.pins.gpio5;
    let spi = Spi::new(peripherals.SPI2, 40u32.MHz(), SpiMode::Mode0).with_pins(
        sck,
        mosi,
        miso,
        gpio::NoPin,
    );
    let cs_output = Output::new(cs, Level::High);
    let mut rst = Output::new(io.pins.gpio33, Level::Low);
    rst.set_high();
    let spi_device = ExclusiveDevice::new_no_delay(spi, cs_output).unwrap();
    let di = SPIInterface::new(spi_device, dc);
    let mut display: mipidsi::Display<
        SPIInterface<ExclusiveDevice<_, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>>,
        GC9A01,
        Output<'_>,
    > = Builder::new(GC9A01, di)
        .invert_colors(ColorInversion::Normal)
        .display_size(240, 240)
        .color_order(ColorOrder::Bgr)
        .invert_colors(ColorInversion::Inverted)
        .reset_pin(rst)
        .init(&mut delay)
        .unwrap();
    display.clear(Rgb565::new(5, 12, 8)).unwrap();
    let cell_size = 10;
    let circle = Circle::new(Point::zero(), 240).into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::new(5, 12, 8))
            .build(),
    );

    circle.draw(&mut display).unwrap();

    let row = 6;
    let col = 6;
    let position = get_cell_position(row, col, cell_size);
    let raw_image =
        ImageRaw::<Rgb565, BigEndian>::new(crate::asset::small::bien_90::test::DATA, 50);
    let image = Image::new(&raw_image, position);
    image.draw(&mut display).unwrap();

    let row = 6;
    let col = 13;
    let position = get_cell_position(row, col, cell_size);
    let raw_image =
        ImageRaw::<Rgb565, BigEndian>::new(crate::asset::small::bien_90::test::DATA, 50);
    let image = Image::new(&raw_image, position);
    image.draw(&mut display).unwrap();

    Text::new(
        "DISCONNECT",
        Point::new(72, 190),
        MonoTextStyle::new(&FONT_10X20, Rgb565::RED),
    )
    .draw(&mut display)
    .unwrap();

    Text::new(
        "17:30",
        Point::new(95, 118),
        MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE),
    )
    .draw(&mut display)
    .unwrap();
    Text::new(
        "10",
        Point::new(20, 180),
        MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE),
    )
    .draw(&mut display)
    .unwrap();
    Text::new(
        "20",
        Point::new(7, 153),
        MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE),
    )
    .draw(&mut display)
    .unwrap();
    Text::new(
        "30",
        Point::new(1, 118),
        MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE),
    )
    .draw(&mut display)
    .unwrap();
    Text::new(
        "40",
        Point::new(7, 85),
        MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE),
    )
    .draw(&mut display)
    .unwrap();
    Text::new(
        "50",
        Point::new(28, 50),
        MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE),
    )
    .draw(&mut display)
    .unwrap();
    Text::new(
        "60",
        Point::new(66, 22),
        MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE),
    )
    .draw(&mut display)
    .unwrap();
    Text::new(
        "70",
        Point::new(112, 13),
        MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE),
    )
    .draw(&mut display)
    .unwrap();
    Text::new(
        "80",
        Point::new(167, 22),
        MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE),
    )
    .draw(&mut display)
    .unwrap();
    Text::new(
        "90",
        Point::new(200, 50),
        MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE),
    )
    .draw(&mut display)
    .unwrap();
    Text::new(
        "100",
        Point::new(213, 85),
        MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE),
    )
    .draw(&mut display)
    .unwrap();
    Text::new(
        "110",
        Point::new(220, 118),
        MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE),
    )
    .draw(&mut display)
    .unwrap();
    Text::new(
        "120",
        Point::new(215, 153),
        MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE),
    )
    .draw(&mut display)
    .unwrap();
    Text::new(
        "130",
        Point::new(205, 180),
        MonoTextStyle::new(&FONT_7X13, Rgb565::WHITE),
    )
    .draw(&mut display)
    .unwrap();

    // BLE
    let mut bluetooth = peripherals.BT;
    let now = || time::now().duration_since_epoch().to_millis();
    loop {
        // BLE
        let connector = BleConnector::new(&init, &mut bluetooth);
        let hci = HciConnector::new(connector, now);
        let mut ble = Ble::new(&hci);
        println!("{:?}", ble.init());
        println!("{:?}", ble.cmd_set_le_advertising_parameters());
        println!(
            "{:?}",
            ble.cmd_set_le_advertising_data(
                create_advertising_data(&[
                    AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
                    AdStructure::ServiceUuids16(&[Uuid::Uuid16(0x1809)]),
                    AdStructure::CompleteLocalName("ducanhkhuong-ble"),
                ])
                .unwrap()
            )
        );
        println!("{:?}", ble.cmd_set_le_advertise_enable(true));
        println!("started advertising");

        let mut rf3 = |_offset: usize, data: &mut [u8]| {
            data[..5].copy_from_slice(&b"Hola!"[..]);
            5
        };
        let mut wf3 = |offset: usize, data: &[u8]| {
            println!("RECEIVED: Offset {}, data {:?}", offset, data);
            let str = core::str::from_utf8(data).unwrap();
            println!("{}", str);
            if str == "connected" {
                *connected.borrow_mut() = true;
            } else if str == "90" {
                *bien_90.borrow_mut() = true;
            }
        };
        gatt!([service {
            uuid: "937312e0-2354-11eb-9f10-fbc30a62cf38",
            characteristics: [characteristic {
                name: "my_characteristic",
                uuid: "987312e0-2354-11eb-9f10-fbc30a62cf38",
                notify: true,
                read: rf3,
                write: wf3,
            },],
        },]);

        let mut rng = bleps::no_rng::NoRng;
        let mut srv = AttributeServer::new(&mut ble, &mut gatt_attributes, &mut rng);

        let mut current_color = Rgb565::BLUE;
        let mut i: f32 = 0.0;
        loop {
            let poll_result = srv.ble.poll();
            let result = match poll_result {
                Some(result) => result,
                None => PollResult::Event(EventType::Unknown),
            };
            match result {
                PollResult::Event(event_type) => match event_type {
                    EventType::ConnectionComplete {
                        status,
                        handle,
                        role,
                        peer_address,
                        interval,
                        latency,
                        timeout,
                    } => {
                        println!("Peer {:?} connected to this node", peer_address);
                        *connected.borrow_mut() = true;
                        break;
                    }
                    _ => {}
                },
                PollResult::AsyncData(async_data) => {}
            }
        }
        loop {
            if *connected.borrow() {
                if *bien_90.borrow() {
                    let row = 15;
                    let col = 8;
                    let position = get_cell_position(row, col, cell_size);
                    let raw_image = ImageRaw::<Rgb565, BigEndian>::new(
                        crate::asset::big::bien_90::test::DATA,
                        80,
                    );
                    let image = Image::new(&raw_image, position);
                    image.draw(&mut display).unwrap();
                    *bien_90.borrow_mut() = false;
                }
                // LCD logic
                let arc = Arc::new(Point::new(32, 30), 174, 135.0.deg(), i.deg()).into_styled(
                    PrimitiveStyleBuilder::new()
                        .stroke_color(current_color)
                        .stroke_width(17)
                        .build(),
                );
                arc.draw(&mut display).unwrap();
                i += 5.0;
                delay.delay_millis(200);
                if i == 270.0 {
                    i = 0.0;
                }

                if i > 186.9 {
                    current_color = Rgb565::new(30, 15, 11);
                } else {
                    current_color = Rgb565::BLUE;
                }
                let arc = Arc::new(
                    Point::new(32, 30),
                    174,
                    (135.0 + i).deg(),
                    270.0.deg() - i.deg(),
                )
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .stroke_color(Rgb565::CSS_GRAY)
                        .stroke_width(17)
                        .build(),
                );
                arc.draw(&mut display).unwrap();
                let mut notification = None;
                if button.is_low() {
                    notification = Some(NotificationData::new(
                        my_characteristic_handle,
                        &b"Notification"[..],
                    ));
                };
                if button.is_high() {
                    debounce_cnt = 500;
                }
                match srv.do_work_with_notification(notification) {
                    Ok(res) => {
                        if let WorkResult::GotDisconnected = res {
                            *connected.borrow_mut() = false;
                            break;
                        }
                    }
                    Err(err) => {
                        println!("{:?}", err);
                    }
                }
            } else {
                i = 0.0;
                let arc = Arc::new(
                    Point::new(32, 30),
                    174,
                    (135.0 + i).deg(),
                    270.0.deg() - i.deg(),
                )
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .stroke_color(Rgb565::CSS_GRAY)
                        .stroke_width(17)
                        .build(),
                );
                arc.draw(&mut display).unwrap();
                Text::new(
                    "DISCONNECT",
                    Point::new(72, 190),
                    MonoTextStyle::new(&FONT_10X20, Rgb565::RED),
                )
                .draw(&mut display)
                .unwrap();
                let notification = None;
                match srv.do_work_with_notification(notification) {
                    Ok(res) => {
                        if let WorkResult::GotDisconnected = res {
                            *connected.borrow_mut() = false;
                            break;
                        }
                    }
                    Err(err) => {
                        println!("{:?}", err);
                    }
                }
            }
        }
    }
}

// Not phần app
// 1 - app phải auto connect (app đảm nhiệm )
// 2 - sau khi connect thành công, app gửi bản tin
