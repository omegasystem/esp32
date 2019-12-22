# [ doc = "Reader of register SPI_SRAM_CMD_REG" ] pub type R = crate :: R < u32 , super :: SPI_SRAM_CMD_REG > ; # [ doc = "Writer for register SPI_SRAM_CMD_REG" ] pub type W = crate :: W < u32 , super :: SPI_SRAM_CMD_REG > ; # [ doc = "Register SPI_SRAM_CMD_REG `reset()`'s with value 0" ] impl crate :: ResetValue for super :: SPI_SRAM_CMD_REG { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `SPI_SRAM_RSTIO`" ] pub type SPI_SRAM_RSTIO_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `SPI_SRAM_RSTIO`" ] pub struct SPI_SRAM_RSTIO_W < 'a > { w : & 'a mut W , } impl < 'a > SPI_SRAM_RSTIO_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 4 ) ) | ( ( ( value as u32 ) & 0x01 ) << 4 ) ; self . w } } # [ doc = "Reader of field `SPI_SRAM_QIO`" ] pub type SPI_SRAM_QIO_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `SPI_SRAM_QIO`" ] pub struct SPI_SRAM_QIO_W < 'a > { w : & 'a mut W , } impl < 'a > SPI_SRAM_QIO_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 1 ) ) | ( ( ( value as u32 ) & 0x01 ) << 1 ) ; self . w } } # [ doc = "Reader of field `SPI_SRAM_DIO`" ] pub type SPI_SRAM_DIO_R = crate :: R < bool , bool > ; # [ doc = "Write proxy for field `SPI_SRAM_DIO`" ] pub struct SPI_SRAM_DIO_W < 'a > { w : & 'a mut W , } impl < 'a > SPI_SRAM_DIO_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x01 ) | ( ( value as u32 ) & 0x01 ) ; self . w } } impl R { # [ doc = "Bit 4 - For SPI0 SRAM IO mode reset enable. SRAM IO mode reset operation will be triggered when the bit is set. The bit will be cleared once the operation done" ] # [ inline ( always ) ] pub fn spi_sram_rstio ( & self ) -> SPI_SRAM_RSTIO_R { SPI_SRAM_RSTIO_R :: new ( ( ( self . bits >> 4 ) & 0x01 ) != 0 ) } # [ doc = "Bit 1 - For SPI0 SRAM QIO mode enable . SRAM QIO enable command will be send when the bit is set. The bit will be cleared once the operation done." ] # [ inline ( always ) ] pub fn spi_sram_qio ( & self ) -> SPI_SRAM_QIO_R { SPI_SRAM_QIO_R :: new ( ( ( self . bits >> 1 ) & 0x01 ) != 0 ) } # [ doc = "Bit 0 - For SPI0 SRAM DIO mode enable . SRAM DIO enable command will be send when the bit is set. The bit will be cleared once the operation done." ] # [ inline ( always ) ] pub fn spi_sram_dio ( & self ) -> SPI_SRAM_DIO_R { SPI_SRAM_DIO_R :: new ( ( self . bits & 0x01 ) != 0 ) } } impl W { # [ doc = "Bit 4 - For SPI0 SRAM IO mode reset enable. SRAM IO mode reset operation will be triggered when the bit is set. The bit will be cleared once the operation done" ] # [ inline ( always ) ] pub fn spi_sram_rstio ( & mut self ) -> SPI_SRAM_RSTIO_W { SPI_SRAM_RSTIO_W { w : self } } # [ doc = "Bit 1 - For SPI0 SRAM QIO mode enable . SRAM QIO enable command will be send when the bit is set. The bit will be cleared once the operation done." ] # [ inline ( always ) ] pub fn spi_sram_qio ( & mut self ) -> SPI_SRAM_QIO_W { SPI_SRAM_QIO_W { w : self } } # [ doc = "Bit 0 - For SPI0 SRAM DIO mode enable . SRAM DIO enable command will be send when the bit is set. The bit will be cleared once the operation done." ] # [ inline ( always ) ] pub fn spi_sram_dio ( & mut self ) -> SPI_SRAM_DIO_W { SPI_SRAM_DIO_W { w : self } } }