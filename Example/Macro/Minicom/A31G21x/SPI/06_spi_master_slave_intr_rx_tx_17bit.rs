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
#    1. SPI0    : PD0 (Slave Select)
#               : PC4 (Serial Clock)
#               : PC2 (Master Out Slave In)
#               : PC3 (Master In Slave Out)
#    2. SPI1    : PE2 (Slave Select)
#               : PE3 (Serial Clock)
#               : PE5 (Master Out Slave In)
#               : PE4 (Master In Slave Out)
#
# E. Port Connection
#    1. PD0 (Slave Select)        <----> PE2 (Slave Select)
#    2. PC4 (Serial Clock)        <----> PE3 (Serial Clock)
#    3. PC2 (Master Out Slave In) <----> PE5 (Master Out Slave In)
#    4. PC3 (Master In Slave Out) <----> PE4 (Master In Slave Out)
#
# For more information, read a user's manual of the target device carefully.
#
# SPI0
# 1. Channel                   : 0 (SPI0)
#
# 2. Config                    : [ 0 i s 17 0 m h 80 -delay 1 1 1 ]
#    Operation                 : i (Interrupt)
#    Mode                      : s (Slave)
#    Data Length               : 17 (17bit)
#    Polarity/Phase            : 0 (Polarity : Low, Phase : Low)
#    Bit Order                 : m (MSB)
#    Slave Select Pin Polarity : h (High)
#    Baudrate                  : 80 (PCLK / (Baudrate + 1))
#    Delay Start               : 1
#    Delay Stop                : 1
#    Delay Burst               : 1
#
# 3. Rx                        : [ 0 5 ]
#    Receive Data Length       : 5
#
# SPI1
# 1. Channel                   : 1 (SPI1)
#
# 2. Config                    : [ 1 i m 17 0 m h 80 -delay 1 1 1 ]
#    Operation                 : i (Interrupt)
#    Mode                      : m (Master)
#    Data Length               : 17 (17bit)
#    Polarity/Phase            : 0 (Polarity : Low, Phase : Low)
#    Bit Order                 : m (MSB)
#    Slave Select Pin Polarity : h (High)
#    Baudrate                  : 80 (PCLK / (Baudrate + 1))
#    Delay Start               : 1
#    Delay Stop                : 1
#    Delay Burst               : 1
#
# 3. Data                      : [ 1 15 0x01 0x5a 0xa5 0x00 0xa5 0x5a 0x01 0xff 0x00 0x00 0x00 0xff 0x01 0x55 0xaa]
#    Data Length               : 15
#    Data                      : 0x01 ... (Hexa and space (delimitor))
#
# 4. Tx                        : [ 1 5 ]
#    Transmit Data Length      : 5
#
# PCU (PEx)
# 1. Port Group                : 4 (PCU Port E)
#
# 2. Port                      : [ 4 2 a 2 ] [ 4 3 a 2 ] [ 4 5 a 2 ] [ 4 4 a 2 ]
#    Pin Number                : 2 / 3 / 5 / 4
#    Pin Mode                  : a (Alternative)
#    Alternative               : 2 (SS1/SCK1/MOSI1/MISO1)
# 
# PCU (PEx)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 2 a 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 3 a 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 5 a 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 4 a 2"
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

send "config 0 i s 17 0 m h 80 -delay 1 1 1"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "rx 0 5"
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

send "config 1 i m 17 0 m h 80 -delay 1 1 1"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "data 1 15 0x01 0x5a 0xa5 0x00 0xa5 0x5a 0x01 0xff 0x00 0x00 0x00 0xff 0x01 0x55 0xaa"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "tx 1 5"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

end:
