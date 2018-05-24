use table::table;

/// nRF24l01 configuration
#[repr(C)]
pub struct Config {
    /// Channel in MHz as offset to 2.4 GHz
    pub channel: u8,
    /// TX addr
    pub txmac: [u8; 5],
    /// RX addr0
    pub mac0: [u8; 5],
    /// RX addr1
    pub mac1: [u8; 5],
    /// RX addr2..5
    pub mac2345: [u8; 4],
    /// Number of addrs to enable
    pub nrmacs: u8,
    /// Expected payload lengths
    pub maclen: [u8; 5],
}

/// Set configuration
pub fn set_config(config: &Config) {
    (table().nrf_config_set)(config as *const Config as *const ());
}

const R_CONFIG_EN_CRC: u8 = 0x08;

/// Start RX mode
pub fn rx(crc: bool) -> Rx {
    let config = if crc {
        R_CONFIG_EN_CRC
    } else {
        0
    };
    (table().nrf_rcv_pkt_start)(config);

    Rx
}

/// Do not construct yourself but use `rx()`
///
/// The instance of this types signifies that the radio is in RX mode.
pub struct Rx;

impl Drop for Rx {
    /// End RX mode
    fn drop(&mut self) {
        (table().nrf_rcv_pkt_end)();
    }
}

/// `Rx.poll()` result
pub enum RxError {
    /// No pkt received
    WouldBlock,
    /// No packet error while receiving
    NoPacket,
    /// Packet too large or corrupted
    Invalid,
}

impl Rx {
    /// Poll for next (non-encrypted) packet
    pub fn poll(&self) -> Result<Payload, RxError> {
        let mut buf = [0u8; 32];
        let maxsize = buf.len();
        let ptr = buf.as_mut_ptr();
        let len = (table().nrf_rcv_pkt_poll)(maxsize as isize, ptr);
        match len {
            _ if len > 0 && len <= maxsize as isize =>
                Ok(Payload::new(len as usize, buf)),
            0 => Err(RxError::WouldBlock),
            -1 => Err(RxError::Invalid),
            -2 => Err(RxError::NoPacket),
            _ => Err(RxError::Invalid),
        }
    }
}

/// Packet payload up to 32 bytes
pub struct Payload {
    buf: [u8; 32],
    len: usize,
}

impl Payload {
    /// Create packet from 32 bytes buffer and a length
    pub fn new(len: usize, buf: [u8; 32]) -> Self {
        Payload { buf, len }
    }
}

impl AsRef<[u8]> for Payload {
    fn as_ref(&self) -> &[u8] {
        &self.buf[0..self.len]
    }
}
