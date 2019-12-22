#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C_SCL_LOW_PERIOD_REG(i)"]
    pub i2c_scl_low_period_reg: I2C_SCL_LOW_PERIOD_REG,
    #[doc = "0x04 - I2C_CTR_REG(i)"]
    pub i2c_ctr_reg: I2C_CTR_REG,
    #[doc = "0x08 - I2C_SR_REG(i)"]
    pub i2c_sr_reg: I2C_SR_REG,
    #[doc = "0x0c - I2C_TO_REG(i)"]
    pub i2c_to_reg: I2C_TO_REG,
    #[doc = "0x10 - I2C_SLAVE_ADDR_REG(i)"]
    pub i2c_slave_addr_reg: I2C_SLAVE_ADDR_REG,
    #[doc = "0x14 - I2C_RXFIFO_ST_REG(i)"]
    pub i2c_rxfifo_st_reg: I2C_RXFIFO_ST_REG,
    #[doc = "0x18 - I2C_FIFO_CONF_REG(i)"]
    pub i2c_fifo_conf_reg: I2C_FIFO_CONF_REG,
    #[doc = "0x1c - I2C_DATA_REG(i)"]
    pub i2c_data_reg: I2C_DATA_REG,
    #[doc = "0x20 - I2C_INT_RAW_REG(i)"]
    pub i2c_int_raw_reg: I2C_INT_RAW_REG,
    #[doc = "0x24 - I2C_INT_CLR_REG(i)"]
    pub i2c_int_clr_reg: I2C_INT_CLR_REG,
    #[doc = "0x28 - I2C_INT_ENA_REG(i)"]
    pub i2c_int_ena_reg: I2C_INT_ENA_REG,
    #[doc = "0x2c - I2C_INT_STATUS_REG(i)"]
    pub i2c_int_status_reg: I2C_INT_STATUS_REG,
    #[doc = "0x30 - I2C_SDA_HOLD_REG(i)"]
    pub i2c_sda_hold_reg: I2C_SDA_HOLD_REG,
    #[doc = "0x34 - I2C_SDA_SAMPLE_REG(i)"]
    pub i2c_sda_sample_reg: I2C_SDA_SAMPLE_REG,
    #[doc = "0x38 - I2C_SCL_HIGH_PERIOD_REG(i)"]
    pub i2c_scl_high_period_reg: I2C_SCL_HIGH_PERIOD_REG,
    _reserved15: [u8; 4usize],
    #[doc = "0x40 - I2C_SCL_START_HOLD_REG(i)"]
    pub i2c_scl_start_hold_reg: I2C_SCL_START_HOLD_REG,
    #[doc = "0x44 - I2C_SCL_RSTART_SETUP_REG(i)"]
    pub i2c_scl_rstart_setup_reg: I2C_SCL_RSTART_SETUP_REG,
    #[doc = "0x48 - I2C_SCL_STOP_HOLD_REG(i)"]
    pub i2c_scl_stop_hold_reg: I2C_SCL_STOP_HOLD_REG,
    #[doc = "0x4c - I2C_SCL_STOP_SETUP_REG(i)"]
    pub i2c_scl_stop_setup_reg: I2C_SCL_STOP_SETUP_REG,
    #[doc = "0x50 - I2C_SCL_FILTER_CFG_REG(i)"]
    pub i2c_scl_filter_cfg_reg: I2C_SCL_FILTER_CFG_REG,
    #[doc = "0x54 - I2C_SDA_FILTER_CFG_REG(i)"]
    pub i2c_sda_filter_cfg_reg: I2C_SDA_FILTER_CFG_REG,
    #[doc = "0x58 - I2C_COMD0_REG(i)"]
    pub i2c_comd0_reg: I2C_COMD0_REG,
    #[doc = "0x5c - I2C_COMD1_REG(i)"]
    pub i2c_comd1_reg: I2C_COMD1_REG,
    #[doc = "0x60 - I2C_COMD2_REG(i)"]
    pub i2c_comd2_reg: I2C_COMD2_REG,
    #[doc = "0x64 - I2C_COMD3_REG(i)"]
    pub i2c_comd3_reg: I2C_COMD3_REG,
    #[doc = "0x68 - I2C_COMD4_REG(i)"]
    pub i2c_comd4_reg: I2C_COMD4_REG,
    #[doc = "0x6c - I2C_COMD5_REG(i)"]
    pub i2c_comd5_reg: I2C_COMD5_REG,
    #[doc = "0x70 - I2C_COMD6_REG(i)"]
    pub i2c_comd6_reg: I2C_COMD6_REG,
    #[doc = "0x74 - I2C_COMD7_REG(i)"]
    pub i2c_comd7_reg: I2C_COMD7_REG,
    #[doc = "0x78 - I2C_COMD8_REG(i)"]
    pub i2c_comd8_reg: I2C_COMD8_REG,
    #[doc = "0x7c - I2C_COMD9_REG(i)"]
    pub i2c_comd9_reg: I2C_COMD9_REG,
    #[doc = "0x80 - I2C_COMD10_REG(i)"]
    pub i2c_comd10_reg: I2C_COMD10_REG,
    #[doc = "0x84 - I2C_COMD11_REG(i)"]
    pub i2c_comd11_reg: I2C_COMD11_REG,
    #[doc = "0x88 - I2C_COMD12_REG(i)"]
    pub i2c_comd12_reg: I2C_COMD12_REG,
    #[doc = "0x8c - I2C_COMD13_REG(i)"]
    pub i2c_comd13_reg: I2C_COMD13_REG,
    #[doc = "0x90 - I2C_COMD14_REG(i)"]
    pub i2c_comd14_reg: I2C_COMD14_REG,
    #[doc = "0x94 - I2C_COMD15_REG(i)"]
    pub i2c_comd15_reg: I2C_COMD15_REG,
    _reserved37: [u8; 96usize],
    #[doc = "0xf8 - I2C_DATE_REG(i)"]
    pub i2c_date_reg: I2C_DATE_REG,
}
#[doc = "I2C_SCL_LOW_PERIOD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_scl_low_period_reg](i2c_scl_low_period_reg) module"]
pub type I2C_SCL_LOW_PERIOD_REG = crate::Reg<u32, _I2C_SCL_LOW_PERIOD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_LOW_PERIOD_REG;
#[doc = "`read()` method returns [i2c_scl_low_period_reg::R](i2c_scl_low_period_reg::R) reader structure"]
impl crate::Readable for I2C_SCL_LOW_PERIOD_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_low_period_reg::W](i2c_scl_low_period_reg::W) writer structure"]
impl crate::Writable for I2C_SCL_LOW_PERIOD_REG {}
#[doc = "I2C_SCL_LOW_PERIOD_REG(i)"]
pub mod i2c_scl_low_period_reg;
#[doc = "I2C_CTR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_ctr_reg](i2c_ctr_reg) module"]
pub type I2C_CTR_REG = crate::Reg<u32, _I2C_CTR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CTR_REG;
#[doc = "`read()` method returns [i2c_ctr_reg::R](i2c_ctr_reg::R) reader structure"]
impl crate::Readable for I2C_CTR_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_ctr_reg::W](i2c_ctr_reg::W) writer structure"]
impl crate::Writable for I2C_CTR_REG {}
#[doc = "I2C_CTR_REG(i)"]
pub mod i2c_ctr_reg;
#[doc = "I2C_SR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_sr_reg](i2c_sr_reg) module"]
pub type I2C_SR_REG = crate::Reg<u32, _I2C_SR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SR_REG;
#[doc = "`read()` method returns [i2c_sr_reg::R](i2c_sr_reg::R) reader structure"]
impl crate::Readable for I2C_SR_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_sr_reg::W](i2c_sr_reg::W) writer structure"]
impl crate::Writable for I2C_SR_REG {}
#[doc = "I2C_SR_REG(i)"]
pub mod i2c_sr_reg;
#[doc = "I2C_TO_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_to_reg](i2c_to_reg) module"]
pub type I2C_TO_REG = crate::Reg<u32, _I2C_TO_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_TO_REG;
#[doc = "`read()` method returns [i2c_to_reg::R](i2c_to_reg::R) reader structure"]
impl crate::Readable for I2C_TO_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_to_reg::W](i2c_to_reg::W) writer structure"]
impl crate::Writable for I2C_TO_REG {}
#[doc = "I2C_TO_REG(i)"]
pub mod i2c_to_reg;
#[doc = "I2C_SLAVE_ADDR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_slave_addr_reg](i2c_slave_addr_reg) module"]
pub type I2C_SLAVE_ADDR_REG = crate::Reg<u32, _I2C_SLAVE_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SLAVE_ADDR_REG;
#[doc = "`read()` method returns [i2c_slave_addr_reg::R](i2c_slave_addr_reg::R) reader structure"]
impl crate::Readable for I2C_SLAVE_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_slave_addr_reg::W](i2c_slave_addr_reg::W) writer structure"]
impl crate::Writable for I2C_SLAVE_ADDR_REG {}
#[doc = "I2C_SLAVE_ADDR_REG(i)"]
pub mod i2c_slave_addr_reg;
#[doc = "I2C_RXFIFO_ST_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_rxfifo_st_reg](i2c_rxfifo_st_reg) module"]
pub type I2C_RXFIFO_ST_REG = crate::Reg<u32, _I2C_RXFIFO_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_RXFIFO_ST_REG;
#[doc = "`read()` method returns [i2c_rxfifo_st_reg::R](i2c_rxfifo_st_reg::R) reader structure"]
impl crate::Readable for I2C_RXFIFO_ST_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_rxfifo_st_reg::W](i2c_rxfifo_st_reg::W) writer structure"]
impl crate::Writable for I2C_RXFIFO_ST_REG {}
#[doc = "I2C_RXFIFO_ST_REG(i)"]
pub mod i2c_rxfifo_st_reg;
#[doc = "I2C_FIFO_CONF_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_fifo_conf_reg](i2c_fifo_conf_reg) module"]
pub type I2C_FIFO_CONF_REG = crate::Reg<u32, _I2C_FIFO_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_FIFO_CONF_REG;
#[doc = "`read()` method returns [i2c_fifo_conf_reg::R](i2c_fifo_conf_reg::R) reader structure"]
impl crate::Readable for I2C_FIFO_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_fifo_conf_reg::W](i2c_fifo_conf_reg::W) writer structure"]
impl crate::Writable for I2C_FIFO_CONF_REG {}
#[doc = "I2C_FIFO_CONF_REG(i)"]
pub mod i2c_fifo_conf_reg;
#[doc = "I2C_DATA_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_data_reg](i2c_data_reg) module"]
pub type I2C_DATA_REG = crate::Reg<u32, _I2C_DATA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_DATA_REG;
#[doc = "`read()` method returns [i2c_data_reg::R](i2c_data_reg::R) reader structure"]
impl crate::Readable for I2C_DATA_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_data_reg::W](i2c_data_reg::W) writer structure"]
impl crate::Writable for I2C_DATA_REG {}
#[doc = "I2C_DATA_REG(i)"]
pub mod i2c_data_reg;
#[doc = "I2C_INT_RAW_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_int_raw_reg](i2c_int_raw_reg) module"]
pub type I2C_INT_RAW_REG = crate::Reg<u32, _I2C_INT_RAW_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_INT_RAW_REG;
#[doc = "`read()` method returns [i2c_int_raw_reg::R](i2c_int_raw_reg::R) reader structure"]
impl crate::Readable for I2C_INT_RAW_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_int_raw_reg::W](i2c_int_raw_reg::W) writer structure"]
impl crate::Writable for I2C_INT_RAW_REG {}
#[doc = "I2C_INT_RAW_REG(i)"]
pub mod i2c_int_raw_reg;
#[doc = "I2C_INT_CLR_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_int_clr_reg](i2c_int_clr_reg) module"]
pub type I2C_INT_CLR_REG = crate::Reg<u32, _I2C_INT_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_INT_CLR_REG;
#[doc = "`read()` method returns [i2c_int_clr_reg::R](i2c_int_clr_reg::R) reader structure"]
impl crate::Readable for I2C_INT_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_int_clr_reg::W](i2c_int_clr_reg::W) writer structure"]
impl crate::Writable for I2C_INT_CLR_REG {}
#[doc = "I2C_INT_CLR_REG(i)"]
pub mod i2c_int_clr_reg;
#[doc = "I2C_INT_ENA_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_int_ena_reg](i2c_int_ena_reg) module"]
pub type I2C_INT_ENA_REG = crate::Reg<u32, _I2C_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_INT_ENA_REG;
#[doc = "`read()` method returns [i2c_int_ena_reg::R](i2c_int_ena_reg::R) reader structure"]
impl crate::Readable for I2C_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_int_ena_reg::W](i2c_int_ena_reg::W) writer structure"]
impl crate::Writable for I2C_INT_ENA_REG {}
#[doc = "I2C_INT_ENA_REG(i)"]
pub mod i2c_int_ena_reg;
#[doc = "I2C_INT_STATUS_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_int_status_reg](i2c_int_status_reg) module"]
pub type I2C_INT_STATUS_REG = crate::Reg<u32, _I2C_INT_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_INT_STATUS_REG;
#[doc = "`read()` method returns [i2c_int_status_reg::R](i2c_int_status_reg::R) reader structure"]
impl crate::Readable for I2C_INT_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_int_status_reg::W](i2c_int_status_reg::W) writer structure"]
impl crate::Writable for I2C_INT_STATUS_REG {}
#[doc = "I2C_INT_STATUS_REG(i)"]
pub mod i2c_int_status_reg;
#[doc = "I2C_SDA_HOLD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_sda_hold_reg](i2c_sda_hold_reg) module"]
pub type I2C_SDA_HOLD_REG = crate::Reg<u32, _I2C_SDA_HOLD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SDA_HOLD_REG;
#[doc = "`read()` method returns [i2c_sda_hold_reg::R](i2c_sda_hold_reg::R) reader structure"]
impl crate::Readable for I2C_SDA_HOLD_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_sda_hold_reg::W](i2c_sda_hold_reg::W) writer structure"]
impl crate::Writable for I2C_SDA_HOLD_REG {}
#[doc = "I2C_SDA_HOLD_REG(i)"]
pub mod i2c_sda_hold_reg;
#[doc = "I2C_SDA_SAMPLE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_sda_sample_reg](i2c_sda_sample_reg) module"]
pub type I2C_SDA_SAMPLE_REG = crate::Reg<u32, _I2C_SDA_SAMPLE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SDA_SAMPLE_REG;
#[doc = "`read()` method returns [i2c_sda_sample_reg::R](i2c_sda_sample_reg::R) reader structure"]
impl crate::Readable for I2C_SDA_SAMPLE_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_sda_sample_reg::W](i2c_sda_sample_reg::W) writer structure"]
impl crate::Writable for I2C_SDA_SAMPLE_REG {}
#[doc = "I2C_SDA_SAMPLE_REG(i)"]
pub mod i2c_sda_sample_reg;
#[doc = "I2C_SCL_HIGH_PERIOD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_scl_high_period_reg](i2c_scl_high_period_reg) module"]
pub type I2C_SCL_HIGH_PERIOD_REG = crate::Reg<u32, _I2C_SCL_HIGH_PERIOD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_HIGH_PERIOD_REG;
#[doc = "`read()` method returns [i2c_scl_high_period_reg::R](i2c_scl_high_period_reg::R) reader structure"]
impl crate::Readable for I2C_SCL_HIGH_PERIOD_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_high_period_reg::W](i2c_scl_high_period_reg::W) writer structure"]
impl crate::Writable for I2C_SCL_HIGH_PERIOD_REG {}
#[doc = "I2C_SCL_HIGH_PERIOD_REG(i)"]
pub mod i2c_scl_high_period_reg;
#[doc = "I2C_SCL_START_HOLD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_scl_start_hold_reg](i2c_scl_start_hold_reg) module"]
pub type I2C_SCL_START_HOLD_REG = crate::Reg<u32, _I2C_SCL_START_HOLD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_START_HOLD_REG;
#[doc = "`read()` method returns [i2c_scl_start_hold_reg::R](i2c_scl_start_hold_reg::R) reader structure"]
impl crate::Readable for I2C_SCL_START_HOLD_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_start_hold_reg::W](i2c_scl_start_hold_reg::W) writer structure"]
impl crate::Writable for I2C_SCL_START_HOLD_REG {}
#[doc = "I2C_SCL_START_HOLD_REG(i)"]
pub mod i2c_scl_start_hold_reg;
#[doc = "I2C_SCL_RSTART_SETUP_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_scl_rstart_setup_reg](i2c_scl_rstart_setup_reg) module"]
pub type I2C_SCL_RSTART_SETUP_REG = crate::Reg<u32, _I2C_SCL_RSTART_SETUP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_RSTART_SETUP_REG;
#[doc = "`read()` method returns [i2c_scl_rstart_setup_reg::R](i2c_scl_rstart_setup_reg::R) reader structure"]
impl crate::Readable for I2C_SCL_RSTART_SETUP_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_rstart_setup_reg::W](i2c_scl_rstart_setup_reg::W) writer structure"]
impl crate::Writable for I2C_SCL_RSTART_SETUP_REG {}
#[doc = "I2C_SCL_RSTART_SETUP_REG(i)"]
pub mod i2c_scl_rstart_setup_reg;
#[doc = "I2C_SCL_STOP_HOLD_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_scl_stop_hold_reg](i2c_scl_stop_hold_reg) module"]
pub type I2C_SCL_STOP_HOLD_REG = crate::Reg<u32, _I2C_SCL_STOP_HOLD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_STOP_HOLD_REG;
#[doc = "`read()` method returns [i2c_scl_stop_hold_reg::R](i2c_scl_stop_hold_reg::R) reader structure"]
impl crate::Readable for I2C_SCL_STOP_HOLD_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_stop_hold_reg::W](i2c_scl_stop_hold_reg::W) writer structure"]
impl crate::Writable for I2C_SCL_STOP_HOLD_REG {}
#[doc = "I2C_SCL_STOP_HOLD_REG(i)"]
pub mod i2c_scl_stop_hold_reg;
#[doc = "I2C_SCL_STOP_SETUP_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_scl_stop_setup_reg](i2c_scl_stop_setup_reg) module"]
pub type I2C_SCL_STOP_SETUP_REG = crate::Reg<u32, _I2C_SCL_STOP_SETUP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_STOP_SETUP_REG;
#[doc = "`read()` method returns [i2c_scl_stop_setup_reg::R](i2c_scl_stop_setup_reg::R) reader structure"]
impl crate::Readable for I2C_SCL_STOP_SETUP_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_stop_setup_reg::W](i2c_scl_stop_setup_reg::W) writer structure"]
impl crate::Writable for I2C_SCL_STOP_SETUP_REG {}
#[doc = "I2C_SCL_STOP_SETUP_REG(i)"]
pub mod i2c_scl_stop_setup_reg;
#[doc = "I2C_SCL_FILTER_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_scl_filter_cfg_reg](i2c_scl_filter_cfg_reg) module"]
pub type I2C_SCL_FILTER_CFG_REG = crate::Reg<u32, _I2C_SCL_FILTER_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SCL_FILTER_CFG_REG;
#[doc = "`read()` method returns [i2c_scl_filter_cfg_reg::R](i2c_scl_filter_cfg_reg::R) reader structure"]
impl crate::Readable for I2C_SCL_FILTER_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_scl_filter_cfg_reg::W](i2c_scl_filter_cfg_reg::W) writer structure"]
impl crate::Writable for I2C_SCL_FILTER_CFG_REG {}
#[doc = "I2C_SCL_FILTER_CFG_REG(i)"]
pub mod i2c_scl_filter_cfg_reg;
#[doc = "I2C_SDA_FILTER_CFG_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_sda_filter_cfg_reg](i2c_sda_filter_cfg_reg) module"]
pub type I2C_SDA_FILTER_CFG_REG = crate::Reg<u32, _I2C_SDA_FILTER_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SDA_FILTER_CFG_REG;
#[doc = "`read()` method returns [i2c_sda_filter_cfg_reg::R](i2c_sda_filter_cfg_reg::R) reader structure"]
impl crate::Readable for I2C_SDA_FILTER_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_sda_filter_cfg_reg::W](i2c_sda_filter_cfg_reg::W) writer structure"]
impl crate::Writable for I2C_SDA_FILTER_CFG_REG {}
#[doc = "I2C_SDA_FILTER_CFG_REG(i)"]
pub mod i2c_sda_filter_cfg_reg;
#[doc = "I2C_COMD0_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd0_reg](i2c_comd0_reg) module"]
pub type I2C_COMD0_REG = crate::Reg<u32, _I2C_COMD0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD0_REG;
#[doc = "`read()` method returns [i2c_comd0_reg::R](i2c_comd0_reg::R) reader structure"]
impl crate::Readable for I2C_COMD0_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd0_reg::W](i2c_comd0_reg::W) writer structure"]
impl crate::Writable for I2C_COMD0_REG {}
#[doc = "I2C_COMD0_REG(i)"]
pub mod i2c_comd0_reg;
#[doc = "I2C_COMD1_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd1_reg](i2c_comd1_reg) module"]
pub type I2C_COMD1_REG = crate::Reg<u32, _I2C_COMD1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD1_REG;
#[doc = "`read()` method returns [i2c_comd1_reg::R](i2c_comd1_reg::R) reader structure"]
impl crate::Readable for I2C_COMD1_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd1_reg::W](i2c_comd1_reg::W) writer structure"]
impl crate::Writable for I2C_COMD1_REG {}
#[doc = "I2C_COMD1_REG(i)"]
pub mod i2c_comd1_reg;
#[doc = "I2C_COMD2_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd2_reg](i2c_comd2_reg) module"]
pub type I2C_COMD2_REG = crate::Reg<u32, _I2C_COMD2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD2_REG;
#[doc = "`read()` method returns [i2c_comd2_reg::R](i2c_comd2_reg::R) reader structure"]
impl crate::Readable for I2C_COMD2_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd2_reg::W](i2c_comd2_reg::W) writer structure"]
impl crate::Writable for I2C_COMD2_REG {}
#[doc = "I2C_COMD2_REG(i)"]
pub mod i2c_comd2_reg;
#[doc = "I2C_COMD3_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd3_reg](i2c_comd3_reg) module"]
pub type I2C_COMD3_REG = crate::Reg<u32, _I2C_COMD3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD3_REG;
#[doc = "`read()` method returns [i2c_comd3_reg::R](i2c_comd3_reg::R) reader structure"]
impl crate::Readable for I2C_COMD3_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd3_reg::W](i2c_comd3_reg::W) writer structure"]
impl crate::Writable for I2C_COMD3_REG {}
#[doc = "I2C_COMD3_REG(i)"]
pub mod i2c_comd3_reg;
#[doc = "I2C_COMD4_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd4_reg](i2c_comd4_reg) module"]
pub type I2C_COMD4_REG = crate::Reg<u32, _I2C_COMD4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD4_REG;
#[doc = "`read()` method returns [i2c_comd4_reg::R](i2c_comd4_reg::R) reader structure"]
impl crate::Readable for I2C_COMD4_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd4_reg::W](i2c_comd4_reg::W) writer structure"]
impl crate::Writable for I2C_COMD4_REG {}
#[doc = "I2C_COMD4_REG(i)"]
pub mod i2c_comd4_reg;
#[doc = "I2C_COMD5_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd5_reg](i2c_comd5_reg) module"]
pub type I2C_COMD5_REG = crate::Reg<u32, _I2C_COMD5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD5_REG;
#[doc = "`read()` method returns [i2c_comd5_reg::R](i2c_comd5_reg::R) reader structure"]
impl crate::Readable for I2C_COMD5_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd5_reg::W](i2c_comd5_reg::W) writer structure"]
impl crate::Writable for I2C_COMD5_REG {}
#[doc = "I2C_COMD5_REG(i)"]
pub mod i2c_comd5_reg;
#[doc = "I2C_COMD6_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd6_reg](i2c_comd6_reg) module"]
pub type I2C_COMD6_REG = crate::Reg<u32, _I2C_COMD6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD6_REG;
#[doc = "`read()` method returns [i2c_comd6_reg::R](i2c_comd6_reg::R) reader structure"]
impl crate::Readable for I2C_COMD6_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd6_reg::W](i2c_comd6_reg::W) writer structure"]
impl crate::Writable for I2C_COMD6_REG {}
#[doc = "I2C_COMD6_REG(i)"]
pub mod i2c_comd6_reg;
#[doc = "I2C_COMD7_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd7_reg](i2c_comd7_reg) module"]
pub type I2C_COMD7_REG = crate::Reg<u32, _I2C_COMD7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD7_REG;
#[doc = "`read()` method returns [i2c_comd7_reg::R](i2c_comd7_reg::R) reader structure"]
impl crate::Readable for I2C_COMD7_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd7_reg::W](i2c_comd7_reg::W) writer structure"]
impl crate::Writable for I2C_COMD7_REG {}
#[doc = "I2C_COMD7_REG(i)"]
pub mod i2c_comd7_reg;
#[doc = "I2C_COMD8_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd8_reg](i2c_comd8_reg) module"]
pub type I2C_COMD8_REG = crate::Reg<u32, _I2C_COMD8_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD8_REG;
#[doc = "`read()` method returns [i2c_comd8_reg::R](i2c_comd8_reg::R) reader structure"]
impl crate::Readable for I2C_COMD8_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd8_reg::W](i2c_comd8_reg::W) writer structure"]
impl crate::Writable for I2C_COMD8_REG {}
#[doc = "I2C_COMD8_REG(i)"]
pub mod i2c_comd8_reg;
#[doc = "I2C_COMD9_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd9_reg](i2c_comd9_reg) module"]
pub type I2C_COMD9_REG = crate::Reg<u32, _I2C_COMD9_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD9_REG;
#[doc = "`read()` method returns [i2c_comd9_reg::R](i2c_comd9_reg::R) reader structure"]
impl crate::Readable for I2C_COMD9_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd9_reg::W](i2c_comd9_reg::W) writer structure"]
impl crate::Writable for I2C_COMD9_REG {}
#[doc = "I2C_COMD9_REG(i)"]
pub mod i2c_comd9_reg;
#[doc = "I2C_COMD10_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd10_reg](i2c_comd10_reg) module"]
pub type I2C_COMD10_REG = crate::Reg<u32, _I2C_COMD10_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD10_REG;
#[doc = "`read()` method returns [i2c_comd10_reg::R](i2c_comd10_reg::R) reader structure"]
impl crate::Readable for I2C_COMD10_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd10_reg::W](i2c_comd10_reg::W) writer structure"]
impl crate::Writable for I2C_COMD10_REG {}
#[doc = "I2C_COMD10_REG(i)"]
pub mod i2c_comd10_reg;
#[doc = "I2C_COMD11_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd11_reg](i2c_comd11_reg) module"]
pub type I2C_COMD11_REG = crate::Reg<u32, _I2C_COMD11_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD11_REG;
#[doc = "`read()` method returns [i2c_comd11_reg::R](i2c_comd11_reg::R) reader structure"]
impl crate::Readable for I2C_COMD11_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd11_reg::W](i2c_comd11_reg::W) writer structure"]
impl crate::Writable for I2C_COMD11_REG {}
#[doc = "I2C_COMD11_REG(i)"]
pub mod i2c_comd11_reg;
#[doc = "I2C_COMD12_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd12_reg](i2c_comd12_reg) module"]
pub type I2C_COMD12_REG = crate::Reg<u32, _I2C_COMD12_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD12_REG;
#[doc = "`read()` method returns [i2c_comd12_reg::R](i2c_comd12_reg::R) reader structure"]
impl crate::Readable for I2C_COMD12_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd12_reg::W](i2c_comd12_reg::W) writer structure"]
impl crate::Writable for I2C_COMD12_REG {}
#[doc = "I2C_COMD12_REG(i)"]
pub mod i2c_comd12_reg;
#[doc = "I2C_COMD13_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd13_reg](i2c_comd13_reg) module"]
pub type I2C_COMD13_REG = crate::Reg<u32, _I2C_COMD13_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD13_REG;
#[doc = "`read()` method returns [i2c_comd13_reg::R](i2c_comd13_reg::R) reader structure"]
impl crate::Readable for I2C_COMD13_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd13_reg::W](i2c_comd13_reg::W) writer structure"]
impl crate::Writable for I2C_COMD13_REG {}
#[doc = "I2C_COMD13_REG(i)"]
pub mod i2c_comd13_reg;
#[doc = "I2C_COMD14_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd14_reg](i2c_comd14_reg) module"]
pub type I2C_COMD14_REG = crate::Reg<u32, _I2C_COMD14_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD14_REG;
#[doc = "`read()` method returns [i2c_comd14_reg::R](i2c_comd14_reg::R) reader structure"]
impl crate::Readable for I2C_COMD14_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd14_reg::W](i2c_comd14_reg::W) writer structure"]
impl crate::Writable for I2C_COMD14_REG {}
#[doc = "I2C_COMD14_REG(i)"]
pub mod i2c_comd14_reg;
#[doc = "I2C_COMD15_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_comd15_reg](i2c_comd15_reg) module"]
pub type I2C_COMD15_REG = crate::Reg<u32, _I2C_COMD15_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_COMD15_REG;
#[doc = "`read()` method returns [i2c_comd15_reg::R](i2c_comd15_reg::R) reader structure"]
impl crate::Readable for I2C_COMD15_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_comd15_reg::W](i2c_comd15_reg::W) writer structure"]
impl crate::Writable for I2C_COMD15_REG {}
#[doc = "I2C_COMD15_REG(i)"]
pub mod i2c_comd15_reg;
#[doc = "I2C_DATE_REG(i)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_date_reg](i2c_date_reg) module"]
pub type I2C_DATE_REG = crate::Reg<u32, _I2C_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_DATE_REG;
#[doc = "`read()` method returns [i2c_date_reg::R](i2c_date_reg::R) reader structure"]
impl crate::Readable for I2C_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [i2c_date_reg::W](i2c_date_reg::W) writer structure"]
impl crate::Writable for I2C_DATE_REG {}
#[doc = "I2C_DATE_REG(i)"]
pub mod i2c_date_reg;