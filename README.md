# Conv
Utility for converting numbers between bases and printing bytes in human readable format.

## Usage:
```
 $ conv 0x1000
 conversions:
        dec: 4096
        hex: 0x1000
        oct: 0o10000
        bin: 0b1000000000000

 4KiB

 $ conv 123456
 conversions:
        dec: 123456
        hex: 0x1e240
        oct: 0o361100
        bin: 0b11110001001000000

 120KiB 576B

 $ conv 1GiB
 conversions:
        dec: 1073741824
        hex: 0x40000000
        oct: 0o10000000000
        bin: 0b1000000000000000000000000000000

 1GiB
```
