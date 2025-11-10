use super::SeesawDeviceInit;
use crate::{
    modules::{
        gpio::{GpioModule, PinMode},
        encoder::EncoderModule,
        status::StatusModule,
        HardwareId,
    },
    seesaw_device, Driver, SeesawError,
};

const SS_SWITCH_SELECT: u8 = 1;
const SS_SWITCH_UP: u8 = 2;
const SS_SWITCH_LEFT: u8 = 3;
const SS_SWITCH_DOWN: u8 = 4;
const SS_SWITCH_RIGHT: u8 = 5;

seesaw_device! {
  /// NeoKey1x4
  name: AnoRotary,
  hardware_id: HardwareId::ATTINY817,
  product_id: 4980,
  default_addr: 0x49
}

impl<D: Driver> GpioModule<D> for AnoRotary<D> {}
impl<D: Driver> EncoderModule<D, 1> for AnoRotary<D> {
    const ENCODER_BTN_PINS: [u8; 1] = [1];
}

impl<D: Driver> SeesawDeviceInit<D> for AnoRotary<D> {
    fn init(mut self) -> Result<Self, SeesawError<D::Error>> {
        self.reset_and_verify_seesaw()?;
        // self.enable_button_pins()?;
        self.set_pin_mode(SS_SWITCH_UP, PinMode::InputPullup)?;
        self.set_pin_mode(SS_SWITCH_DOWN, PinMode::InputPullup)?;
        self.set_pin_mode(SS_SWITCH_LEFT, PinMode::InputPullup)?;
        self.set_pin_mode(SS_SWITCH_RIGHT, PinMode::InputPullup)?;
        self.set_pin_mode(SS_SWITCH_SELECT, PinMode::InputPullup)?;

        self.enable_button(0)?;
        Ok(self)
    }
}

impl<D: Driver> AnoRotary<D> {
    pub fn keys(&mut self) -> Result<u8, SeesawError<D::Error>> {
        self.digital_read_bulk().map(|r| ((r >> 2) ^ 0xF) as u8)
    }

    pub fn all_keys(&mut self) -> Result<u16, SeesawError<D::Error>> {
        // 0x1F = right 5 bits = 11111
        self.digital_read_bulk().map(|r| ((r >> 1) ^ 0b11111) as u16)
    }
}