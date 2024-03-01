pub mod param {
    pub const IKE_FLOAT_STORED_AETHER_MUL : f32 = 1.2;
    pub const IKE_INT_STORED_AETHER_EFFECT_FRAME : i32 = 5;
}
pub mod vars {
    pub mod instance {//0x0???
        //flag
        pub const IKE_FLAG_STORED_AETHER : i32 = 0x0000;
        pub const IKE_FLAG_STORED_AETHER_EFFECT_DISABLE : i32 = 0x0001;
        //int
        pub const IKE_INT_STORED_AETHER_EFFECT_COUNT : i32 = 0x0000;
        //float
        pub const IKE_FLOAT_SPECIAL_N_CHARGE_COUNT : i32 = 0x0000;
    }
    // pub mod status {//0x1???
    //     //flag
    //     pub const IKE_FLAG_ : i32 = 0x0000;
    //     //int
    //     pub const IKE_INT_ : i32 = 0x0000;
    //     //float
    //     pub const IKE_FLOAT_ : i32 = 0x0000;
    // }
}
