/**
Mach header
      magic cputype cpusubtype  caps    filetype ncmds sizeofcmds      flags
 0xfeedfacf 16777223          3  0x80           2    15       2080 0x00a18085
**/
const MACH_HEADER_64_DATA: [u8; 32] = [0xcf, 0xfa, 0xed, 0xfe, 0x7, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0,
                                       0x80, 0x2, 0x0, 0x0, 0x0, 0xf, 0x0, 0x0, 0x0, 0x20, 0x8,
                                       0x0, 0x0, 0x85, 0x80, 0xa1, 0x0, 0x0, 0x0, 0x0, 0x0];

/**
Load command 0
      cmd LC_SEGMENT_64
  cmdsize 72
  segname __PAGEZERO
   vmaddr 0x0000000000000000
   vmsize 0x0000000100000000
  fileoff 0
 filesize 0
  maxprot 0x00000000
 initprot 0x00000000
   nsects 0
    flags 0x0
**/
const LC_SEGMENT_64_PAGEZERO_DATA: [u8; 0x48] = [0x19, 0x0, 0x0, 0x0, 0x48, 0x0, 0x0, 0x0, 0x5f,
                                                 0x5f, 0x50, 0x41, 0x47, 0x45, 0x5a, 0x45, 0x52,
                                                 0x4f, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                                 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                                 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                                 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                                 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                                 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];

/**
Load command 1
      cmd LC_SEGMENT_64
  cmdsize 712
  segname __TEXT
   vmaddr 0x0000000100000000
   vmsize 0x00000000001e3000
  fileoff 0
 filesize 1978368
  maxprot 0x00000007
 initprot 0x00000005
   nsects 8
    flags 0x0
Section
  sectname __text
   segname __TEXT
      addr 0x0000000100001090
      size 0x00000000001668e0
    offset 4240
     align 2^4 (16)
    reloff 0
    nreloc 0
     flags 0x80000400
 reserved1 0
 reserved2 0
Section
  sectname __stubs
   segname __TEXT
      addr 0x0000000100167970
      size 0x00000000000001e0
    offset 1472880
     align 2^1 (2)
    reloff 0
    nreloc 0
     flags 0x80000408
 reserved1 0 (index into indirect symbol table)
 reserved2 6 (size of stubs)
Section
  sectname __stub_helper
   segname __TEXT
      addr 0x0000000100167b50
      size 0x0000000000000330
    offset 1473360
     align 2^2 (4)
    reloff 0
    nreloc 0
     flags 0x80000400
 reserved1 0
 reserved2 0
Section
  sectname __gcc_except_tab
   segname __TEXT
      addr 0x0000000100167e80
      size 0x0000000000010d40
    offset 1474176
     align 2^2 (4)
    reloff 0
    nreloc 0
     flags 0x00000000
 reserved1 0
 reserved2 0
Section
  sectname __const
   segname __TEXT
      addr 0x0000000100178bc0
      size 0x0000000000027ec0
    offset 1543104
     align 2^6 (64)
    reloff 0
    nreloc 0
     flags 0x00000000
 reserved1 0
 reserved2 0
Section
  sectname __cstring
   segname __TEXT
      addr 0x00000001001a0a80
      size 0x00000000000018b8
    offset 1706624
     align 2^0 (1)
    reloff 0
    nreloc 0
     flags 0x00000002
 reserved1 0
 reserved2 0
Section
  sectname __unwind_info
   segname __TEXT
      addr 0x00000001001a2338
      size 0x0000000000004edc
    offset 1712952
     align 2^2 (4)
    reloff 0
    nreloc 0
     flags 0x00000000
 reserved1 0
 reserved2 0
Section
  sectname __eh_frame
   segname __TEXT
      addr 0x00000001001a7218
      size 0x000000000003bdc0
    offset 1733144
     align 2^3 (8)
    reloff 0
    nreloc 0
     flags 0x00000000
 reserved1 0
 reserved2 0
**/
const LC_SEGMENT_64_TEXT_DATA: [u8; 0x2c8] = [0x19, 0x0, 0x0, 0x0, 0xc8, 0x2, 0x0, 0x0, 0x5f,
                                              0x5f, 0x54, 0x45, 0x58, 0x54, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x1, 0x0, 0x0, 0x0, 0x0, 0x30, 0x1e, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x30, 0x1e, 0x0, 0x0, 0x0, 0x0, 0x0, 0x7, 0x0,
                                              0x0, 0x0, 0x5, 0x0, 0x0, 0x0, 0x8, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x74, 0x65, 0x78,
                                              0x74, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x5f, 0x5f, 0x54, 0x45, 0x58, 0x54, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x90, 0x10,
                                              0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0xe0, 0x68, 0x16, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x90, 0x10, 0x0, 0x0, 0x4, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x4, 0x0, 0x80, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x73,
                                              0x74, 0x75, 0x62, 0x73, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x54, 0x45, 0x58,
                                              0x54, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x70, 0x79, 0x16, 0x0, 0x1, 0x0, 0x0, 0x0,
                                              0xe0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x70, 0x79,
                                              0x16, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x8, 0x4, 0x0, 0x80, 0x0, 0x0,
                                              0x0, 0x0, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x5f, 0x5f, 0x73, 0x74, 0x75, 0x62, 0x5f, 0x68,
                                              0x65, 0x6c, 0x70, 0x65, 0x72, 0x0, 0x0, 0x0, 0x5f,
                                              0x5f, 0x54, 0x45, 0x58, 0x54, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x50, 0x7b, 0x16, 0x0,
                                              0x1, 0x0, 0x0, 0x0, 0x30, 0x3, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x50, 0x7b, 0x16, 0x0, 0x2, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x4,
                                              0x0, 0x80, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x67, 0x63, 0x63,
                                              0x5f, 0x65, 0x78, 0x63, 0x65, 0x70, 0x74, 0x5f,
                                              0x74, 0x61, 0x62, 0x5f, 0x5f, 0x54, 0x45, 0x58,
                                              0x54, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x80, 0x7e, 0x16, 0x0, 0x1, 0x0, 0x0, 0x0,
                                              0x40, 0xd, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x80, 0x7e,
                                              0x16, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x5f, 0x5f, 0x63, 0x6f, 0x6e, 0x73, 0x74, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x54,
                                              0x45, 0x58, 0x54, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0xc0, 0x8b, 0x17, 0x0, 0x1, 0x0, 0x0,
                                              0x0, 0xc0, 0x7e, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0xc0,
                                              0x8b, 0x17, 0x0, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x5f, 0x5f, 0x63, 0x73, 0x74, 0x72, 0x69, 0x6e,
                                              0x67, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f,
                                              0x54, 0x45, 0x58, 0x54, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x80, 0xa, 0x1a, 0x0, 0x1,
                                              0x0, 0x0, 0x0, 0xb8, 0x18, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x80, 0xa, 0x1a, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x5f, 0x5f, 0x75, 0x6e, 0x77, 0x69,
                                              0x6e, 0x64, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x0, 0x0,
                                              0x0, 0x5f, 0x5f, 0x54, 0x45, 0x58, 0x54, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x38, 0x23,
                                              0x1a, 0x0, 0x1, 0x0, 0x0, 0x0, 0xdc, 0x4e, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x38, 0x23, 0x1a, 0x0, 0x2, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x65,
                                              0x68, 0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x54, 0x45, 0x58,
                                              0x54, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x18, 0x72, 0x1a, 0x0, 0x1, 0x0, 0x0, 0x0,
                                              0xc0, 0xbd, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x18,
                                              0x72, 0x1a, 0x0, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0];

/**
Load command 2
      cmd LC_SEGMENT_64
  cmdsize 872
  segname __DATA
   vmaddr 0x00000001001e3000
   vmsize 0x0000000000013000
  fileoff 1978368
 filesize 73728
  maxprot 0x00000007
 initprot 0x00000003
   nsects 10
    flags 0x0
Section
  sectname __nl_symbol_ptr
   segname __DATA
      addr 0x00000001001e3000
      size 0x0000000000000010
    offset 1978368
     align 2^3 (8)
    reloff 0
    nreloc 0
     flags 0x00000006
 reserved1 80 (index into indirect symbol table)
 reserved2 0
Section
  sectname __got
   segname __DATA
      addr 0x00000001001e3010
      size 0x0000000000000028
    offset 1978384
     align 2^3 (8)
    reloff 0
    nreloc 0
     flags 0x00000006
 reserved1 82 (index into indirect symbol table)
 reserved2 0
Section
  sectname __la_symbol_ptr
   segname __DATA
      addr 0x00000001001e3038
      size 0x0000000000000280
    offset 1978424
     align 2^3 (8)
    reloff 0
    nreloc 0
     flags 0x00000007
 reserved1 87 (index into indirect symbol table)
 reserved2 0
Section
  sectname __mod_init_func
   segname __DATA
      addr 0x00000001001e32b8
      size 0x0000000000000008
    offset 1979064
     align 2^3 (8)
    reloff 0
    nreloc 0
     flags 0x00000009
 reserved1 0
 reserved2 0
Section
  sectname __const
   segname __DATA
      addr 0x00000001001e32c0
      size 0x00000000000109c0
    offset 1979072
     align 2^4 (16)
    reloff 0
    nreloc 0
     flags 0x00000000
 reserved1 0
 reserved2 0
Section
  sectname __data
   segname __DATA
      addr 0x00000001001f3c80
      size 0x00000000000003f8
    offset 2047104
     align 2^4 (16)
    reloff 0
    nreloc 0
     flags 0x00000000
 reserved1 0
 reserved2 0
Section
  sectname __thread_vars
   segname __DATA
      addr 0x00000001001f4078
      size 0x0000000000000090
    offset 2048120
     align 2^0 (1)
    reloff 0
    nreloc 0
     flags 0x00000013
 reserved1 0
 reserved2 0
Section
  sectname __thread_data
   segname __DATA
      addr 0x00000001001f4108
      size 0x00000000000000c0
    offset 2048264
     align 2^3 (8)
    reloff 0
    nreloc 0
     flags 0x00000011
 reserved1 0
 reserved2 0
Section
  sectname __common
   segname __DATA
      addr 0x00000001001f41d0
      size 0x0000000000000ea8
    offset 0
     align 2^4 (16)
    reloff 0
    nreloc 0
     flags 0x00000001
 reserved1 0
 reserved2 0
Section
  sectname __bss
   segname __DATA
      addr 0x00000001001f5080
      size 0x0000000000000190
    offset 0
     align 2^4 (16)
    reloff 0
    nreloc 0
     flags 0x00000001
 reserved1 0
 reserved2 0
**/
const LC_SEGMENT_64_DATA_DATA: [u8; 0x368] = [0x19, 0x0, 0x0, 0x0, 0x68, 0x3, 0x0, 0x0, 0x5f,
                                              0x5f, 0x44, 0x41, 0x54, 0x41, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x30, 0x1e, 0x0,
                                              0x1, 0x0, 0x0, 0x0, 0x0, 0x30, 0x1, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x30, 0x1e, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x20, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x7, 0x0,
                                              0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0xa, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x6e, 0x6c, 0x5f,
                                              0x73, 0x79, 0x6d, 0x62, 0x6f, 0x6c, 0x5f, 0x70,
                                              0x74, 0x72, 0x0, 0x5f, 0x5f, 0x44, 0x41, 0x54, 0x41,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x30, 0x1e, 0x0, 0x1, 0x0, 0x0, 0x0, 0x10, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x30, 0x1e, 0x0,
                                              0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x6, 0x0, 0x0, 0x0, 0x50, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f,
                                              0x67, 0x6f, 0x74, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x44, 0x41, 0x54,
                                              0x41, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x10, 0x30, 0x1e, 0x0, 0x1, 0x0, 0x0, 0x0,
                                              0x28, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x10, 0x30,
                                              0x1e, 0x0, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x6, 0x0, 0x0, 0x0, 0x52, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x5f, 0x5f, 0x6c, 0x61, 0x5f, 0x73, 0x79, 0x6d,
                                              0x62, 0x6f, 0x6c, 0x5f, 0x70, 0x74, 0x72, 0x0, 0x5f,
                                              0x5f, 0x44, 0x41, 0x54, 0x41, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x38, 0x30, 0x1e, 0x0,
                                              0x1, 0x0, 0x0, 0x0, 0x80, 0x2, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x38, 0x30, 0x1e, 0x0, 0x3, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x7, 0x0,
                                              0x0, 0x0, 0x57, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x6d, 0x6f, 0x64,
                                              0x5f, 0x69, 0x6e, 0x69, 0x74, 0x5f, 0x66, 0x75,
                                              0x6e, 0x63, 0x0, 0x5f, 0x5f, 0x44, 0x41, 0x54, 0x41,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0xb8, 0x32, 0x1e, 0x0, 0x1, 0x0, 0x0, 0x0, 0x8, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xb8, 0x32, 0x1e, 0x0,
                                              0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f,
                                              0x63, 0x6f, 0x6e, 0x73, 0x74, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x44, 0x41,
                                              0x54, 0x41, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0xc0, 0x32, 0x1e, 0x0, 0x1, 0x0, 0x0, 0x0,
                                              0xc0, 0x9, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0xc0, 0x32,
                                              0x1e, 0x0, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x5f, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x44,
                                              0x41, 0x54, 0x41, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x80, 0x3c, 0x1f, 0x0, 0x1, 0x0, 0x0,
                                              0x0, 0xf8, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x80,
                                              0x3c, 0x1f, 0x0, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x5f, 0x5f, 0x74, 0x68, 0x72, 0x65, 0x61, 0x64,
                                              0x5f, 0x76, 0x61, 0x72, 0x73, 0x0, 0x0, 0x0, 0x5f,
                                              0x5f, 0x44, 0x41, 0x54, 0x41, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x78, 0x40, 0x1f, 0x0,
                                              0x1, 0x0, 0x0, 0x0, 0x90, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x78, 0x40, 0x1f, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x13, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x74, 0x68, 0x72,
                                              0x65, 0x61, 0x64, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x0,
                                              0x0, 0x0, 0x5f, 0x5f, 0x44, 0x41, 0x54, 0x41, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x8,
                                              0x41, 0x1f, 0x0, 0x1, 0x0, 0x0, 0x0, 0xc0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x8, 0x41, 0x1f, 0x0, 0x3,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x11, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x63,
                                              0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x44, 0x41, 0x54,
                                              0x41, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0xd0, 0x41, 0x1f, 0x0, 0x1, 0x0, 0x0, 0x0,
                                              0xa8, 0xe, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x5f, 0x5f, 0x62, 0x73, 0x73, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5f, 0x5f, 0x44,
                                              0x41, 0x54, 0x41, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x80, 0x50, 0x1f, 0x0, 0x1, 0x0, 0x0,
                                              0x0, 0x90, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0,
                                              0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                              0x0];

/**
Load command 3
      cmd LC_SEGMENT_64
  cmdsize 72
  segname __LINKEDIT
   vmaddr 0x00000001001f6000
   vmsize 0x000000000017a000
  fileoff 2052096
 filesize 1544372
  maxprot 0x00000007
 initprot 0x00000001
   nsects 0
    flags 0x0
**/
const LC_SEGMENT_64_LINKEDIT_DATA: [u8; 0x48] = [0x19, 0x0, 0x0, 0x0, 0x48, 0x0, 0x0, 0x0, 0x5f,
                                                 0x5f, 0x4c, 0x49, 0x4e, 0x4b, 0x45, 0x44, 0x49,
                                                 0x54, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x60,
                                                 0x1f, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0xa0, 0x17,
                                                 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x50, 0x1f, 0x0,
                                                 0x0, 0x0, 0x0, 0x0, 0xb4, 0x90, 0x17, 0x0, 0x0,
                                                 0x0, 0x0, 0x0, 0x7, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0,
                                                 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];

/**
Load command 4
            cmd LC_DYLD_INFO_ONLY
        cmdsize 48
     rebase_off 2052096
    rebase_size 3368
       bind_off 2055464
      bind_size 80
  weak_bind_off 2055544
 weak_bind_size 24
  lazy_bind_off 2055568
 lazy_bind_size 1688
     export_off 2057256
    export_size 34856
**/
const LC_DYLD_INFO_ONLY_DATA: [u8; 0x30] = [0x22, 0x0, 0x0, 0x80, 0x30, 0x0, 0x0, 0x0, 0x0, 0x50,
                                            0x1f, 0x0, 0x28, 0xd, 0x0, 0x0, 0x28, 0x5d, 0x1f, 0x0,
                                            0x50, 0x0, 0x0, 0x0, 0x78, 0x5d, 0x1f, 0x0, 0x18, 0x0,
                                            0x0, 0x0, 0x90, 0x5d, 0x1f, 0x0, 0x98, 0x6, 0x0, 0x0,
                                            0x28, 0x64, 0x1f, 0x0, 0x28, 0x88, 0x0, 0x0];

/**
Load command 5
     cmd LC_SYMTAB
 cmdsize 24
  symoff 2100616
   nsyms 36797
  stroff 2690036
 strsize 906432
**/
const LC_SYMTAB_DATA: [u8; 0x18] = [0x2, 0x0, 0x0, 0x0, 0x18, 0x0, 0x0, 0x0, 0x88, 0xd, 0x20, 0x0,
                                    0xbd, 0x8f, 0x0, 0x0, 0xf4, 0xb, 0x29, 0x0, 0xc0, 0xd4, 0xd,
                                    0x0];

/**
Load command 6
            cmd LC_DYSYMTAB
        cmdsize 80
      ilocalsym 0
      nlocalsym 35968
     iextdefsym 35968
     nextdefsym 746
      iundefsym 36714
      nundefsym 83
         tocoff 0
           ntoc 0
      modtaboff 0
        nmodtab 0
   extrefsymoff 0
    nextrefsyms 0
 indirectsymoff 2689368
  nindirectsyms 167
      extreloff 0
        nextrel 0
      locreloff 0
        nlocrel 0
**/
const LC_DYSYMTAB_DATA: [u8; 0x50] = [0xb, 0x0, 0x0, 0x0, 0x50, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                      0x80, 0x8c, 0x0, 0x0, 0x80, 0x8c, 0x0, 0x0, 0xea, 0x2, 0x0,
                                      0x0, 0x6a, 0x8f, 0x0, 0x0, 0x53, 0x0, 0x0, 0x0, 0x0, 0x0,
                                      0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                      0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x58, 0x9,
                                      0x29, 0x0, 0xa7, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                                      0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];

/**
Load command 7
          cmd LC_LOAD_DYLINKER
      cmdsize 32
         name /usr/lib/dyld (offset 12)
**/
const LC_LOAD_DYLINKER_DATA: [u8; 0x20] = [0xe, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0xc, 0x0, 0x0,
                                           0x0, 0x2f, 0x75, 0x73, 0x72, 0x2f, 0x6c, 0x69, 0x62,
                                           0x2f, 0x64, 0x79, 0x6c, 0x64, 0x0, 0x0, 0x0, 0x0, 0x0,
                                           0x0, 0x0];

/**
Load command 8
     cmd LC_UUID
 cmdsize 24
    uuid 92E3CF1F-20DA-3373-A98C-851366D353BF
**/
const LC_UUID_DATA: [u8; 0x18] = [0x1b, 0x0, 0x0, 0x0, 0x18, 0x0, 0x0, 0x0, 0x92, 0xe3, 0xcf,
                                  0x1f, 0x20, 0xda, 0x33, 0x73, 0xa9, 0x8c, 0x85, 0x13, 0x66,
                                  0xd3, 0x53, 0xbf];

/**
Load command 9
      cmd LC_VERSION_MIN_MACOSX
  cmdsize 16
  version 10.11
      sdk 10.11
**/
const LC_VERSION_MIN_MACOSX_DATA: [u8; 0x10] = [0x24, 0x0, 0x0, 0x0, 0x10, 0x0, 0x0, 0x0, 0x0,
                                                0xb, 0xa, 0x0, 0x0, 0xb, 0xa, 0x0];


/**
Load command 10
      cmd LC_SOURCE_VERSION
  cmdsize 16
  version 0.0
**/
const LC_SOURCE_VERSION_DATA: [u8; 0x10] = [0x2a, 0x0, 0x0, 0x0, 0x10, 0x0, 0x0, 0x0, 0x0, 0x0,
                                            0x0, 0x0, 0x0, 0x0, 0x0, 0x0];


/**
Load command 11
       cmd LC_MAIN
   cmdsize 24
  entryoff 70656
 stacksize 0
**/
const LC_MAIN_DATA: [u8; 0x18] = [0x28, 0x0, 0x0, 0x80, 0x18, 0x0, 0x0, 0x0, 0x0, 0x14, 0x1, 0x0,
                                  0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];

/**
Load command 12
          cmd LC_LOAD_DYLIB
      cmdsize 56
         name /usr/lib/libSystem.B.dylib (offset 24)
   time stamp 2 Thu Jan  1 08:00:02 1970
      current version 1226.10.1
compatibility version 1.0.0
**/
const LC_LOAD_DYLIB_DATA: [u8; 0x38] = [0xc, 0x0, 0x0, 0x0, 0x38, 0x0, 0x0, 0x0, 0x18, 0x0, 0x0,
                                        0x0, 0x2, 0x0, 0x0, 0x0, 0x1, 0xa, 0xca, 0x4, 0x0, 0x0,
                                        0x1, 0x0, 0x2f, 0x75, 0x73, 0x72, 0x2f, 0x6c, 0x69, 0x62,
                                        0x2f, 0x6c, 0x69, 0x62, 0x53, 0x79, 0x73, 0x74, 0x65,
                                        0x6d, 0x2e, 0x42, 0x2e, 0x64, 0x79, 0x6c, 0x69, 0x62, 0x0,
                                        0x0, 0x0, 0x0, 0x0, 0x0];

/**
Load command 13
      cmd LC_FUNCTION_STARTS
  cmdsize 16
  dataoff 2092112
 datasize 8504
**/
const LC_FUNCTION_STARTS_DATA: [u8; 0x10] = [0x26, 0x0, 0x0, 0x0, 0x10, 0x0, 0x0, 0x0, 0x50, 0xec,
                                             0x1f, 0x0, 0x38, 0x21, 0x0, 0x0];

/**
Load command 14
      cmd LC_DATA_IN_CODE
  cmdsize 16
  dataoff 2100616
 datasize 0
**/
const LC_DATA_IN_CODE_DATA: [u8; 0x10] = [0x29, 0x0, 0x0, 0x0, 0x10, 0x0, 0x0, 0x0, 0x88, 0xd,
                                          0x20, 0x0, 0x0, 0x0, 0x0, 0x0];

pub fn prepare_test_mach_header() -> Vec<u8> {
    let mut header = Vec::new();

    header.extend_from_slice(&MACH_HEADER_64_DATA[..]);

    header.extend_from_slice(&LC_SEGMENT_64_PAGEZERO_DATA[..]);
    header.extend_from_slice(&LC_SEGMENT_64_TEXT_DATA[..]);
    header.extend_from_slice(&LC_SEGMENT_64_DATA_DATA[..]);
    header.extend_from_slice(&LC_SEGMENT_64_LINKEDIT_DATA[..]);
    header.extend_from_slice(&LC_DYLD_INFO_ONLY_DATA[..]);
    header.extend_from_slice(&LC_SYMTAB_DATA[..]);
    header.extend_from_slice(&LC_DYSYMTAB_DATA[..]);
    header.extend_from_slice(&LC_LOAD_DYLINKER_DATA[..]);
    header.extend_from_slice(&LC_UUID_DATA[..]);
    header.extend_from_slice(&LC_VERSION_MIN_MACOSX_DATA[..]);
    header.extend_from_slice(&LC_SOURCE_VERSION_DATA[..]);
    header.extend_from_slice(&LC_MAIN_DATA[..]);
    header.extend_from_slice(&LC_LOAD_DYLIB_DATA[..]);
    header.extend_from_slice(&LC_FUNCTION_STARTS_DATA[..]);
    header.extend_from_slice(&LC_DATA_IN_CODE_DATA[..]);

    header
}
