# A. Description
#    A list of commands here configures SPI module and receives data via SPI.
#
# B. Preparation
#    Connecting ports with an external SPI device should be correct.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. SPI
#    2. PCU/GPIO
#
# D. Default Port
#    1. SPI0    : PA4 (Slave Select)
#               : PA5 (Serial Clock)
#               : PA7 (Master Out Slave In)
#               : PA6 (Master In Slave Out)
#    2. SPI1    : PB12 (Slave Select)
#               : PB13 (Serial Clock)
#               : PB15 (Master Out Slave In)
#               : PB14 (Master In Slave Out)
#
# E. Port Connection
#    1. PA4 (Slave Select)        <----> PB12 (Slave Select)
#    2. PA5 (Serial Clock)        <----> PB13 (Serial Clock)
#    3. PA7 (Master Out Slave In) <----> PB15 (Master Out Slave In)
#    4. PA6 (Master In Slave Out) <----> PB14 (Master In Slave Out)
#
# For more information, read a user's manual of the target device carefully.
#
# SPI0
# 1. Channel                   : 0 (SPI0)
#
# 2. Config                    : [ 0 d s 8 0 m h 5 -delay 1 1 1 ]
#    Operation                 : d (DMA)
#    Mode                      : s (Slave)
#    Data Length               : 8 (8bit)
#    Polarity/Phase            : 0 (Polarity : Low, Phase : Low)
#    Bit Order                 : m (MSB)
#    Slave Select Pin Polarity : h (High)
#    Baudrate                  : 5 (PCLK / (Baudrate + 1))
#    Delay Start               : 1
#    Delay Stop                : 1
#    Delay Burst               : 1
#
# 3. Rx                        : [ 0 8 ]
#    Receive Data Length       : 8
#
# SPI1
# 1. Channel                   : 1 (SPI1)
#
# 2. Config                    : [ 1 d m 8 0 m h 5 -delay 1 1 1 ]
#    Operation                 : d (DMA)
#    Mode                      : m (Master)
#    Data Length               : 8 (8bit)
#    Polarity/Phase            : 0 (Polarity : Low, Phase : Low)
#    Bit Order                 : m (MSB)
#    Slave Select Pin Polarity : h (High)
#    Baudrate                  : 5 (PCLK / (Baudrate + 1))
#    Delay Start               : 1
#    Delay Stop                : 1
#    Delay Burst               : 1
#
# 3. Data                      : [ 1 8 0x01 0x02 0x03 0x04 0x05 0x06 0x07 0x09 ]
#    Data Length               : 8
#    Data                      : 0x01 ... (Hexa and space (delimitor))
#
# 4. Tx                        : [ 1 8 ]
#    Transmit Data Length      : 8
#
# PCU (PBx)
# 1. Port Group                : 1 (PCU Port B)
#
# 2. Port                      : [ 1 12 a 0 ] [ 1 13 a 0 ] [ 1 15 a 0 ] [ 1 14 a 0 ]
#    Pin Number                : 12 / 13 / 15 / 14
#    Pin Mode                  : a (Alternative)
#    Alternative               : 0 (SS1/SCK1/MOSI1/MISO1)
# 
# PCU (PBx)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 12 a 0"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 13 a 0"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 15 a 0"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 14 a 0"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# SPI0
send "cm spi"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "config 0 d s 8 0 m h 5 -delay 1 1 1"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "rx 0 8"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}


# SPI1
send "cm spi"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "uninit 1"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "init 1"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "config 1 d m 8 0 m h 5 -delay 1 1 1"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "data 1 8 0xa5 0x5a 0xa5 0x5a 0xff 0x10 0x01 0x55"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "tx 1 8"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

end:
