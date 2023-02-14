use alloc::string::String;
use crate::inputs::InputConfig;

pub struct BinaryConfig
{
    pub id: String,
    pub pin: u16,
}

impl InputConfig<bool> for BinaryConfig {}