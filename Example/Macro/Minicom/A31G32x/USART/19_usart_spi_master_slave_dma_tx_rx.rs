# A. Description
#    A list of commands here configures USART as SPI and receives data.
#
# B. Preparation
#    Connecting ports with a suitable instrument should be correct
#
# C. Prerequisite Example (abov_example_config.h)
#    1. USART
#    2. PCU/GPIO
#
# D. Default Port
#    1. USART1  : PA1 (Slave Select)
#               : PA4 (Serial Clock)
#               : PA2 (Master Out Slave In)
#               : PA3 (Master In Slave Out)
#    2. USART2  : PB14 (Slave Select)
#               : PB12 (Serial Clock)
#               : PB10 (Master Out Slave In)
#               : PB11 (Master In Slave Out)
#
# E. Port Connection
#    1. PA1 (Slave Select)        <----> PB14 (Slave Select)
#    2. PA4 (Serial Clock)        <----> PB12 (Serial Clock)
#    3. PA2 (Master Out Slave In) <----> PB11 (Master In Slave Out)
#    4. PA3 (Master In Slave Out) <----> PB10 (Master Out Slave In)
#
# For more information, read a user's manual of the target device carefully.
#
# USART1
# 1. Channel                   : 1 (USART1)
#
# 2. Config                    : [ 1 d s m m l e -swap 11 ]
#    Operation                 : d (DMA)
#    USART Mode                : s (SPI)
#    SPI Mode                  : s (Slave)
#    Bit Order                 : m (MSB)
#    Polarity                  : l (Low)
#    Phase                     : e (Even Edge) (Odd Edge not working)
#    Swap MOSI/MISO port       : Swap
#    Baudrate                  : 11
#
# 3. Data                      : [ 1 8 0xff 0xa5 0x5a 0xff 0x01 0x10 0x55 0xaa ]
#    Data Length               : 8
#    Data                      : 0xa5 ... (Hexa and space (delimitor))
#
# 4. Tx                        : [ 1 8 ]
#    Receive Data Length       : 8
#
# USART2
# 1. Channel                   : 2 (USART2)
#
# 2. Config                    : [ 2 d s s m l e -swap 6 ]
#    Operation                 : d (DMA)
#    USART Mode                : s (SPI)
#    SPI Mode                  : s (Slave)
#    Bit Order                 : m (MSB)
#    Polarity                  : l (Low)
#    Phase                     : e (Even Edge) (Odd Edge not working)
#    Baudrate                  : 6
#
# 3. Rx                        : [ 2 8 ]
#    Receive Data Length       : 8
#
# PCU (PAx)
# 1. Port Group                : 0 (PCU Port A)
#
# 2. Port                      : [ 0 1 a 2 ] [ 0 4 a 2 ] [ 0 2 a 2 ] [ 0 3 a 2 ]
#    Pin Number                : 1 / 4 / 2 / 3
#    Pin Mode                  : a (Alternative)
#    Alternative               : 2 (SS/SCK/MOSI/MISO)
#
# PCU (PBx)
# 1. Port Group                : 1 (PCU Port B)
#
# 2. Port                      : [ 1 14 a 1 ] [ 1 12 a 1 ] [ 1 10 a 1 ] [ 1 11 a 1 ]
#    Pin Number                : 14 / 12 / 10 / 11
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 (SS/SCK/MOSI/MISO)
#
# PCU (PAx)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 1 a 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 4 a 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 2 a 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 3 a 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# PCU (PBx)
send "port 1 14 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 12 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 10 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 11 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# USART2
send "cm usart"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

send "uninit 2"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

send "init 2"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

send "config 2 d s s m l e -swap 11"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

send "rx 2 8"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}


# USART1
send "cm usart"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

send "uninit 1"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

send "init 1"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

send "config 1 d s m m l e 11"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

send "data 1 8 0xff 0xa5 0x5a 0xff 0x01 0x10 0x55 0xaa"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

send "tx 1 8"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

end:
