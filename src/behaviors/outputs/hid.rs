use crate::behaviors::{Behavior, BehaviorConfig};

struct HID
{
    behavior: fn() -> (),
}

impl Behavior for HID
{
    fn execute(&self) -> () {
        (self.behavior)()
    }
}

struct HidConfig(u8);

impl BehaviorConfig<HID> for HidConfig {}