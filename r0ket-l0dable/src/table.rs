/// Firmware call table linked to `0x0000_0124`
#[allow(non_snake_case)]
#[repr(C)]
pub struct Table {
    pub DoChar: extern "C" fn(isize, isize, isize) -> isize,
    pub DoInt: extern "C" fn(isize, isize, isize) -> isize,
    pub DoIntX: extern "C" fn(isize, isize, u8) -> isize,
    pub DoString: extern "C" fn(isize, isize, *const u8) -> isize,
    pub IntToStr: extern "C" fn(isize, usize, i8) -> *const u8,
    pub IntToStrX: extern "C" fn(usize, usize) -> *const u8,
    pub Font_7x8: *const (),
    pub font: *const (),
    pub getFontHeight: extern "C" fn() -> isize,
    pub setExtFont: extern "C" fn(*const u8),
    pub lcdBuffer: *mut u8,
    pub lcdClear: extern "C" fn(),
    pub lcdDisplay: extern "C" fn(),
    pub lcdFill: extern "C" fn(u8),
    pub lcdGetPixel: extern "C" fn(i8, i8) -> bool,
    pub lcdLoadImage: extern "C" fn(*const u8),
    pub lcdNl: extern "C" fn(),
    pub lcdPrint: extern "C" fn(*const u8),
    pub lcdCheckNl: extern "C" fn(),
    pub notimplemented1: extern "C" fn(),
    pub lcdPrintln: extern "C" fn(*const u8),
    pub lcdRefresh: extern "C" fn(),
    pub lcdSetPixel: extern "C" fn(i8, i8, bool),
    pub lcdShift: extern "C" fn(isize, isize, bool),
    pub lcdShowAnim: extern "C" fn(*const u8, u32) -> u8,
    pub getInput: extern "C" fn() -> u8,
    pub getInputRaw: extern "C" fn() -> u8,
    pub getInputWait: extern "C" fn() -> u8,
    pub getInputWaitRelease: extern "C" fn(),
    pub getInputWaitTimeout: extern "C" fn(isize) -> u8,
    pub xxtea_decode_words: extern "C" fn(),
    pub xxtea_encode_words: extern "C" fn(),
    pub ECIES_decryptkeygen: extern "C" fn(),
    pub ECIES_encyptkeygen: extern "C" fn(),
    pub bitstr_parse_export: extern "C" fn(),
    pub f_close: extern "C" fn(),
    pub notimplemented2: extern "C" fn(),
    pub f_lseek: extern "C" fn(),
    pub f_open: extern "C" fn(),
    pub f_read: extern "C" fn(),
    pub f_write: extern "C" fn(),
    pub _nrfresets: *const (),
    pub nrf_check_reset: extern "C" fn(),
    pub nrf_config_set: extern "C" fn(*const ()),
    pub nrf_rcv_pkt_end: extern "C" fn(),
    pub nrf_rcv_pkt_poll_dec: extern "C" fn(isize, *const u8, *const u32) -> isize,
    pub nrf_rcv_pkt_start: extern "C" fn(i8),
    pub nrf_rcv_pkt_time_encr: extern "C" fn(isize, isize, *const u8, *const u32) -> isize,
    pub nrf_read_reg: extern "C" fn(u8) -> u8,
    pub nrf_snd_pkt_crc_encr: extern "C" fn(isize, *const u8, *const u32) -> i8,
    pub delayms: extern "C" fn(u32),
    pub delayms_queue: extern "C" fn(u32),
    pub delayms_queue_plus: extern "C" fn(u32, u8) -> u8,
    pub push_queue: extern "C" fn(),
    pub the_queue: *const (),
    pub work_queue: extern "C" fn(),
    pub readFile: extern "C" fn(),
    pub selectFile: extern "C" fn(),
    pub writeFile: extern "C" fn(),
    pub saveConfig: extern "C" fn(),
    pub the_config: *const (),
    pub gpioGetValue: extern "C" fn(),
    pub gpioSetDir: extern "C" fn(),
    pub gpioSetValue: extern "C" fn(),
    pub sspReceive: extern "C" fn(),
    pub sspSend: extern "C" fn(),
    pub sspSendReceive: extern "C" fn(),
    pub meshGetMessage: extern "C" fn(),
    pub meshbuffer: *const (),
    pub meshgen: *const (),
    pub meshincctr: *const (),
    pub meshnice: *const (),
    pub strcpy: extern "C" fn(),
    pub strlen: extern "C" fn(),
    pub memmove: extern "C" fn(),
    pub memset: extern "C" fn(),
    pub GetLight: extern "C" fn(),
    pub GetUUID32: extern "C" fn(),
    pub GetVoltage: extern "C" fn() -> u32,
    pub isNight: extern "C" fn(),
    pub _timectr: *const (),
    pub crc16: extern "C" fn(),
    pub getRandom: extern "C" fn() -> u32,
    pub getSeconds: extern "C" fn() -> u32,
    pub iapReadSerialNumber: extern "C" fn(),
    pub input: extern "C" fn(),
    pub handleMenu: extern "C" fn(),
    pub menuflags: *const (),
    pub mygmtime: extern "C" fn(),
    pub nickfont: *const (),
    pub nickname: *const (),
    pub systickGetTicks: extern "C" fn() -> u32,
    pub uint32touint8p: extern "C" fn(),
    pub uint8ptouint32: extern "C" fn(),
    pub I2CMasterBuffer: *const (),
    pub I2CSlaveBuffer: *const (),
    pub I2CWriteLength: *const (),
    pub I2CReadLength: *const (),
    pub i2cEngine: extern "C" fn(),
    pub i2cInit: extern "C" fn(),
    pub timer32Callback0: *const (),
    pub lcdRead: extern "C" fn(),
    pub lcdInit: extern "C" fn(),
    pub lcdSetCrsr: extern "C" fn(),
    pub lcdSetCrsrX: extern "C" fn(),
    pub getInputWaitRepeat: extern "C" fn(),
    pub applyConfig: extern "C" fn(),
    pub flameDetect: extern "C" fn(),
    pub flameSetBrightness: extern "C" fn(),
    pub flameSetColor: extern "C" fn(),
    pub flameClaim: extern "C" fn(),
    pub flameFree: extern "C" fn(),
    pub o_init: extern "C" fn(),
    pub o_path_new: extern "C" fn(),
    pub o_move_to: extern "C" fn(),
    pub o_line_to: extern "C" fn(),
    pub o_curve_to: extern "C" fn(),
    pub o_close: extern "C" fn(),
    pub o_set_gray: extern "C" fn(),
    pub o_fill: extern "C" fn(),
    pub o_set_shader: extern "C" fn(),
    pub o_identity: extern "C" fn(),
    pub o_transform: extern "C" fn(),
    pub getrelease: extern "C" fn(),
    pub nrf_set_strength: extern "C" fn(),
    pub nrf_rcv_pkt_poll: extern "C" fn(),
    pub nrf_cmd: extern "C" fn(),
    pub nrf_write_reg: extern "C" fn(),
    pub nrf_snd_pkt: extern "C" fn(),
}

#[allow(improper_ctypes)]
extern "C" {
    static TheTable: Table;
}

pub fn table() -> &'static Table {
    unsafe {
        &TheTable
    }
}
