# A. Description
#    A list of commands here configures USART as SPI and transmits data.
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
#
# For more information, read a user's manual of the target device carefully.
#
# USART1
# 1. Channel                   : 1 (USART1)
#
# 2. Config                    : [ 1 i s m m l o 6 ]
#    Operation                 : i (Interrupt)
#    USART Mode                : s (SPI)
#    SPI Mode                  : s (Slave)
#    Bit Order                 : m (MSB)
#    Polarity                  : l (Low)
#    Phase                     : o (Odd Edge)
#    Swap MOSI/MISO port       : Swap
#    Baudrate                  : 6
#
# 3. Data                      : [ 1 8 0xa5 0x5a 0xa5 0x5a 0x05 0x07 0x09 0xa0 ]
#    Data Length               : 8
#    Data                      : 0xa5 ... (Hexa and space (delimitor))
#
# 4. Tx                        : [ 1 8 ]
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

send "config 1 i s m m l e 6"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

send "data 1 8 0xa5 0x5a 0xa5 0x5a 0x05 0x07 0x09 0xa0"
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
