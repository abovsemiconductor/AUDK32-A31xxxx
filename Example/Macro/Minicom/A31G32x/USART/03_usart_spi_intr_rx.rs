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
#
# For more information, read a user's manual of the target device carefully.
#
# USART1
# 1. Channel                   : 1 (USART1)
#
# 2. Config                    : [ 1 i s s m l o -swap 6 ]
#    Operation                 : i (Interrupt)
#    USART Mode                : s (SPI)
#    SPI Mode                  : s (Slave)
#    Bit Order                 : m (MSB)
#    Polarity                  : l (Low)
#    Phase                     : o (Odd Edge)
#    Swap MOSI/MISO port       : Swap
#    Baudrate                  : 6
#
# 3. Rx                        : [ 1 8 ]
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

send "config 1 i s s m l e -swap 6"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

send "rx 1 8"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

end:
