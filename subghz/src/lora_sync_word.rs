/// LoRa synchronization word.
///
/// Argument of [`set_lora_sync_word`][crate::SubGhz::set_lora_sync_word].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LoRaSyncWord {
    /// LoRa private network.
    Private,
    /// LoRa public network.
    Public,
}

impl LoRaSyncWord {
    pub(crate) const fn bytes(self) -> [u8; 2] {
        match self {
            LoRaSyncWord::Private => [0x14, 0x24],
            LoRaSyncWord::Public => [0x34, 0x44],
        }
    }
}
