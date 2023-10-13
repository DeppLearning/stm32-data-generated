
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ipcc",
            extends: None,
            description: Some("IPCC"),
            items: &[BlockItem {
                name: "cpu",
                description: Some("CPU specific registers"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                byte_offset: 0,
                inner: BlockItemInner::Block(BlockItemBlock { block: "IpccCpu" }),
            }],
        },
        Block {
            name: "IpccCpu",
            extends: None,
            description: Some("IPCC"),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some("Control register CPUx"),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("C1cr"),
                    }),
                },
                BlockItem {
                    name: "mr",
                    description: Some("Mask register CPUx"),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("C1mr"),
                    }),
                },
                BlockItem {
                    name: "scr",
                    description: Some("Status Set or Clear register CPU1"),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(Register {
                        access: Access::Write,
                        bit_size: 32,
                        fieldset: Some("C1scr"),
                    }),
                },
                BlockItem {
                    name: "sr",
                    description: Some("CPU1 to CPU2 status register"),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(Register {
                        access: Access::Read,
                        bit_size: 32,
                        fieldset: Some("C1to2sr"),
                    }),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "C2scr",
            extends: None,
            description: Some("Status Set or Clear register CPU2"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chc",
                    description: Some("processor 2 Receive channel x status clear"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 6, stride: 1 })),
                    enumm: None,
                },
                Field {
                    name: "chs",
                    description: Some("processor 2 Transmit channel 1 status set"),
                    bit_offset: 16,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 6, stride: 1 })),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2cr",
            extends: None,
            description: Some("Control register CPU2"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxoie",
                    description: Some("processor 2 Receive channel occupied interrupt enable"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfie",
                    description: Some("processor 2 Transmit channel free interrupt enable"),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C1to2sr",
            extends: None,
            description: Some("CPU1 to CPU2 status register"),
            bit_size: 32,
            fields: &[Field {
                name: "chf",
                description: Some("processor 1 transmit to process 2 Receive channel x status flag"),
                bit_offset: 0,
                bit_size: 1,
                array: Some(Array::Regular(RegularArray { len: 6, stride: 1 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "C2mr",
            extends: None,
            description: Some("Mask register CPU2"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chom",
                    description: Some("processor 2 Receive channel x occupied interrupt enable"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 6, stride: 1 })),
                    enumm: None,
                },
                Field {
                    name: "chfm",
                    description: Some("processor 2 Transmit channel 1 free interrupt mask"),
                    bit_offset: 16,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 6, stride: 1 })),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C1scr",
            extends: None,
            description: Some("Status Set or Clear register CPU1"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chc",
                    description: Some("processor 1 Receive channel x status clear"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 6, stride: 1 })),
                    enumm: None,
                },
                Field {
                    name: "chs",
                    description: Some("processor 1 Transmit channel x status set"),
                    bit_offset: 16,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 6, stride: 1 })),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C1mr",
            extends: None,
            description: Some("Mask register CPU1"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chom",
                    description: Some("processor 1 Receive channel x occupied interrupt enable"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 6, stride: 1 })),
                    enumm: None,
                },
                Field {
                    name: "chfm",
                    description: Some("processor 1 Transmit channel x free interrupt mask"),
                    bit_offset: 16,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 6, stride: 1 })),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2toc1sr",
            extends: None,
            description: Some("CPU2 to CPU1 status register"),
            bit_size: 32,
            fields: &[Field {
                name: "chf",
                description: Some("processor 2 transmit to process 1 Receive channel x status flag"),
                bit_offset: 0,
                bit_size: 1,
                array: Some(Array::Regular(RegularArray { len: 6, stride: 1 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "C1cr",
            extends: None,
            description: Some("Control register CPU1"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxoie",
                    description: Some("processor 1 Receive channel occupied interrupt enable"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfie",
                    description: Some("processor 1 Transmit channel free interrupt enable"),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
