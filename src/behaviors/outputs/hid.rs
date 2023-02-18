use crate::behaviors::outputs::OutputConfig;

struct HID(u8);

impl OutputConfig<u8> for HID {}