MEMORY {
    FLASH (rx)  : ORIGIN = 0x08000000, LENGTH = 1024K
    RAM  (xrw) : ORIGIN = 0x20000000, LENGTH = 0x40000
}

_stack_start = ORIGIN(RAM) + 0x30000;
_stack_end = ORIGIN(RAM) + 0x28000;