
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pwr",
            extends: None,
            description: Some(
                "Power control",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "Power control register 1",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "Power control register 2",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr3",
                    description: Some(
                        "Power control register 3",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr4",
                    description: Some(
                        "Power control register 4",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr1",
                    description: Some(
                        "Power status register 1",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr2",
                    description: Some(
                        "Power status register 2",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "scr",
                    description: Some(
                        "Power status clear register",
                    ),
                    array: None,
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Scr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pucr",
                    description: Some(
                        "Power Port A pull-up control register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdcr",
                    description: Some(
                        "Power Port A pull-down control register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 36,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seccfgr",
                    description: Some(
                        "Power secure configuration register",
                    ),
                    array: None,
                    byte_offset: 120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seccfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privcfgr",
                    description: Some(
                        "Power privilege configuration register",
                    ),
                    array: None,
                    byte_offset: 128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privcfgr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Scr",
            extends: None,
            description: Some(
                "Power status clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cwuf",
                    description: Some(
                        "Clear wakeup flag",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "sbf",
                    description: Some(
                        "Clear standby flag",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr2",
            extends: None,
            description: Some(
                "Power status register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "reglps",
                    description: Some(
                        "Low-power regulator started",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reglpf",
                    description: Some(
                        "Low-power regulator flag",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vosf",
                    description: Some(
                        "Voltage scaling flag",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvdo",
                    description: Some(
                        "Power voltage detector output",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvmo1",
                    description: Some(
                        "Peripheral voltage monitoring output: VDDUSB vs. 1.2 V",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvmo2",
                    description: Some(
                        "Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvmo3",
                    description: Some(
                        "Peripheral voltage monitoring output: VDDA vs. 1.62 V",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvmo4",
                    description: Some(
                        "Peripheral voltage monitoring output: VDDA vs. 2.2 V",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr4",
            extends: None,
            description: Some(
                "Power control register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wp1",
                    description: Some(
                        "Wakeup pin WKUP1 polarity",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wp2",
                    description: Some(
                        "Wakeup pin WKUP2 polarity",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wp3",
                    description: Some(
                        "Wakeup pin WKUP3 polarity",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wp4",
                    description: Some(
                        "Wakeup pin WKUP4 polarity",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wp5",
                    description: Some(
                        "Wakeup pin WKUP5 polarity",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vbe",
                    description: Some(
                        "VBAT battery charging enable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vbrs",
                    description: Some(
                        "VBAT battery charging resistor selection",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smpsbyp",
                    description: Some(
                        "SMPSBYP",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "extsmpsen",
                    description: Some(
                        "EXTSMPSEN",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smpsfsten",
                    description: Some(
                        "SMPSFSTEN",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smpslpen",
                    description: Some(
                        "SMPSLPEN",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pcr",
            extends: None,
            description: Some(
                "Power Port pull control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "p",
                    description: Some(
                        "Port pull bit y (y=0..15)",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Seccfgr",
            extends: None,
            description: Some(
                "Power secure configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wup1sec",
                    description: Some(
                        "WKUP1 pin security",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wup2sec",
                    description: Some(
                        "WKUP2 pin security",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wup3sec",
                    description: Some(
                        "WKUP3 pin security",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wup4sec",
                    description: Some(
                        "WKUP4 pin security",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wup5sec",
                    description: Some(
                        "WKUP5 pin security",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpmsec",
                    description: Some(
                        "LPMSEC",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vdmsec",
                    description: Some(
                        "VDMSEC",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vbsec",
                    description: Some(
                        "VBSEC",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apcsec",
                    description: Some(
                        "APCSEC",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Privcfgr",
            extends: None,
            description: Some(
                "Power privilege configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "priv_",
                    description: Some(
                        "PRIV",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some(
                "Power control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpms",
                    description: Some(
                        "Low-power mode selection",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lpms",
                    ),
                },
                Field {
                    name: "dbp",
                    description: Some(
                        "Disable backup domain write protection",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vos",
                    description: Some(
                        "Voltage scaling range selection",
                    ),
                    bit_offset: 9,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Vos",
                    ),
                },
                Field {
                    name: "lpr",
                    description: Some(
                        "Low-power run",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lpr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Sr1",
            extends: None,
            description: Some(
                "Power status register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cwuf1",
                    description: Some(
                        "Wakeup flag 1",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cwuf2",
                    description: Some(
                        "Wakeup flag 2",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cwuf3",
                    description: Some(
                        "Wakeup flag 3",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cwuf4",
                    description: Some(
                        "Wakeup flag 4",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cwuf5",
                    description: Some(
                        "Wakeup flag 5",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csbf",
                    description: Some(
                        "Standby flag",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wufi",
                    description: Some(
                        "Wakeup flag internal",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr3",
            extends: None,
            description: Some(
                "Power control register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ewup",
                    description: Some(
                        "Enable Wakeup pin WKUP",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rrs",
                    description: Some(
                        "SRAM2 retention in Standby mode",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rrs",
                    ),
                },
                Field {
                    name: "apc",
                    description: Some(
                        "Apply pull-up and pull-down configuration",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ulpmen",
                    description: Some(
                        "ULPMEN",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ucpd_stdby",
                    description: Some(
                        "UCPD_STDBY",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ucpd_dbdis",
                    description: Some(
                        "UCPD_DBDIS",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: Some(
                "Power control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pvde",
                    description: Some(
                        "Power voltage detector enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pls",
                    description: Some(
                        "Power voltage detector level selection",
                    ),
                    bit_offset: 1,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pls",
                    ),
                },
                Field {
                    name: "pvme1",
                    description: Some(
                        "Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvme2",
                    description: Some(
                        "Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvme3",
                    description: Some(
                        "Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvme4",
                    description: Some(
                        "Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iosv",
                    description: Some(
                        "VDDIO2 Independent I/Os supply valid",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usv",
                    description: Some(
                        "VDDUSB USB supply valid",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Lpr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MAINMODE",
                    description: Some(
                        "Voltage regulator in Main mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOWPOWERMODE",
                    description: Some(
                        "Voltage regulator in low-power mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vos",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "RANGE0",
                    description: Some(
                        "Range 0",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RANGE1",
                    description: Some(
                        "Range 1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RANGE2",
                    description: Some(
                        "Range 2",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Pls",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "V2_0",
                    description: Some(
                        "2.0V",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "V2_2",
                    description: Some(
                        "2.2V",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "V2_4",
                    description: Some(
                        "2.4V",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "V2_5",
                    description: Some(
                        "2.5V",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "V2_6",
                    description: Some(
                        "2.6V",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "V2_8",
                    description: Some(
                        "2.8V",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "V2_9",
                    description: Some(
                        "2.9V",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "EXTERNAL",
                    description: Some(
                        "External input analog voltage PVD_IN (compared internally to VREFINT)",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Lpms",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "STOP0",
                    description: Some(
                        "Stop 0 mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STOP1",
                    description: Some(
                        "Stop 1 mode",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "STOP2",
                    description: Some(
                        "Stop 2 mode",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "STANDBY",
                    description: Some(
                        "Standby mode",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "SHUTDOWN",
                    description: Some(
                        "Shutdown mode",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Rrs",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "POWEROFF",
                    description: Some(
                        "SRAM2 powered off in Standby mode (SRAM2 content lost)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ONLPR",
                    description: Some(
                        "SRAM2 powered by the low-power regulator in Standby mode (SRAM2 content kept)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ONLPRTOP4KB",
                    description: Some(
                        "Only the upper 4 Kbytes of SRAM2 are powered by the low-power regulator in Standby mode (upper 4 Kbytes of SRAM2 content 0x2003 F000 - 0x2003 FFFF is kept).",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
