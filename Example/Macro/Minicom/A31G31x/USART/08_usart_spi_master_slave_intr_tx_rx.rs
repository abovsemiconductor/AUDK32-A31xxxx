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
#    1. USART1  : PD5 (Slave Select)
#               : PD4 (Serial Clock)
#               : PD2 (Master Out Slave In)
#               : PD3 (Master In Slave Out)
#    2. USART2  : PE15 (Slave Select)
#               : PE14 (Serial Clock)
#               : PE12 (Master Out Slave In)
#               : PE13 (Master In Slave Out)
#
# E. Port Connection
#    1. PD5 (Slave Select)        <----> PE15 (Slave Select)
#    2. PD4 (Serial Clock)        <----> PE14 (Serial Clock)
#    3. PD2 (Master Out Slave In) <----> PE13 (Master In Slave Out)
#    4. PD3 (Master In Slave Out) <----> PE12 (Master Out Slave In)
#
# For more information, read a user's manual of the target device carefully.
#
# USART1
# 1. Channel                   : 1 (USART1)
#
# 2. Config                    : [ 1 i s m m l o -swap 11 ]
#    Operation                 : i (Interrupt)
#    USART Mode                : s (SPI)
#    SPI Mode                  : s (Slave)
#    Bit Order                 : m (MSB)
#    Polarity                  : l (Low)
#    Phase                     : o (Odd Edge)
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
# 2. Config                    : [ 2 i s s m l o -swap 11 ]
#    Operation                 : i (Interrupt)
#    USART Mode                : s (SPI)
#    SPI Mode                  : s (Slave)
#    Bit Order                 : m (MSB)
#    Polarity                  : l (Low)
#    Phase                     : o (Odd Edge)
#    Baudrate                  : 11
#
# 3. Rx                        : [ 2 8 ]
#    Receive Data Length       : 8
#
# PCU (PDx)
# 1. Port Group                : 3 (PCU Port D)
#
# 2. Port                      : [ 3 5 a 2 ] [ 3 4 a 2 ] [ 3 2 a 2 ] [ 3 3 a 2 ]
#    Pin Number                : 5 / 4 / 2 / 3
#    Pin Mode                  : a (Alternative)
#    Alternative               : 2 (SS/SCK/MOSI/MISO)
#
# PCU (PEx)
# 1. Port Group                : 4 (PCU Port E)
#
# 2. Port                      : [ 4 15 a 2 ] [ 4 14 a 2 ] [ 4 12 a 2 ] [ 4 13 a 2 ]
#    Pin Number                : 15 / 14 / 12 / 13
#    Pin Mode                  : a (Alternative)
#    Alternative               : 2 (SS/SCK/MOSI/MISO)
#
# PCU (PDx)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 5 a 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 4 a 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 2 a 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 3 a 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# PCU (PEx)
send "port 4 15 a 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 14 a 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 12 a 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 13 a 2"
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

send "config 2 i s s m l o -swap 11"
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

send "config 1 i s m m l o 11"
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
