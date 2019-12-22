# [ doc = "Reader of register SYSCON_SARADC_SAR1_PATT_TAB3_REG" ] pub type R = crate :: R < u32 , super :: SYSCON_SARADC_SAR1_PATT_TAB3_REG > ; # [ doc = "Writer for register SYSCON_SARADC_SAR1_PATT_TAB3_REG" ] pub type W = crate :: W < u32 , super :: SYSCON_SARADC_SAR1_PATT_TAB3_REG > ; # [ doc = "Register SYSCON_SARADC_SAR1_PATT_TAB3_REG `reset()`'s with value 0" ] impl crate :: ResetValue for super :: SYSCON_SARADC_SAR1_PATT_TAB3_REG { type Type = u32 ; # [ inline ( always ) ] fn reset_value ( ) -> Self :: Type { 0 } } # [ doc = "Reader of field `SYSCON_SARADC_SAR1_PATT_TAB3`" ] pub type SYSCON_SARADC_SAR1_PATT_TAB3_R = crate :: R < u32 , u32 > ; # [ doc = "Write proxy for field `SYSCON_SARADC_SAR1_PATT_TAB3`" ] pub struct SYSCON_SARADC_SAR1_PATT_TAB3_W < 'a > { w : & 'a mut W , } impl < 'a > SYSCON_SARADC_SAR1_PATT_TAB3_W < 'a > { # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u32 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0xffff_ffff ) | ( ( value as u32 ) & 0xffff_ffff ) ; self . w } } impl R { # [ doc = "Bits 0:31 - Item 8 ~ 11 for pattern table 1 (each item one byte)" ] # [ inline ( always ) ] pub fn syscon_saradc_sar1_patt_tab3 ( & self ) -> SYSCON_SARADC_SAR1_PATT_TAB3_R { SYSCON_SARADC_SAR1_PATT_TAB3_R :: new ( ( self . bits & 0xffff_ffff ) as u32 ) } } impl W { # [ doc = "Bits 0:31 - Item 8 ~ 11 for pattern table 1 (each item one byte)" ] # [ inline ( always ) ] pub fn syscon_saradc_sar1_patt_tab3 ( & mut self ) -> SYSCON_SARADC_SAR1_PATT_TAB3_W { SYSCON_SARADC_SAR1_PATT_TAB3_W { w : self } } }