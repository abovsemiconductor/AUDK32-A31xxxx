# A. Description
#    A list of commands here configures USART as UART and receives data.
#
# B. Preparation
#    Connecting ports with a suitable instrument should be correct
#
# C. Prerequisite Example (abov_example_config.h)
#    1. USART
#    2. PCU/GPIO
#
# D. Default Port
#    1. USART1  : PD2 (Transmit)
#               : PD3 (Receive)
#    2. USART2  : PE12 (Transmit)
#               : PE13 (Receive)
#
# E. Port Connection
#    1. PD2 (Transmit) <----> PE13 (Receive)
#    2. PD3 (Receive)  <----> PE12 (Transmit)
#
# F. Note
#    Should change a value of CONFIG_IRQ_26 to 0  /* 0 : USART12, 1 : USART13, 2 : SPI21 */ at config_a31g22x.h.
#
# For more information, read a user's manual of the target device carefully.
#
# USART1
# 1. Channel                   : 1 (USART1)
#
# 2. Config                    : [ 1 i u 9 n 1 n 38400 ]
#    Operation                 : i (Interrupt)
#    USART Mode                : u (UART)
#    Data Bit                  : 9 (9bit)
#    Parity                    : n (None)
#    Stop Bit                  : 1 (1bit)
#    Speed Mode                : n (Normal)
#    Baudrate                  : 38400 bps
#
# 3. Data                      : [ 1 8 0xa5 0x5a 0xa5 0x5a 0x05 0x07 0x09 0xa0 ]
#    Data Length               : 8
#    Data                      : 0xa5 ... (Hexa and space (delimitor))
#
# 4. Tx                        : [ 1 8 ]
#    Receive Data Length       : 8
#
# USART2
# 1. Channel                   : 2 (USART2)
#
# 2. Config                    : [ 2 i u 9 n 1 n 38400 ]
#    Operation                 : i (Interrupt)
#    USART Mode                : u (UART)
#    Data Bit                  : 9 (9bit)
#    Parity                    : n (None)
#    Stop Bit                  : 1 (1bit)
#    Speed Mode                : n (Normal)
#    Baudrate                  : 38400 bps
#
# 3. Rx                        : [ 2 8 ]
#    Receive Data Length       : 8
#
# PCU (PDx)
# 1. Port Group                : 3 (PCU Port D)
#
# 2. Port                      : [ 3 2 a 1 -pupd p ] [ 3 3 a 1 -pupd p ]
#    Pin Number                : 2 / 3
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 (Tx/Rx)
#    Pull-up/down              : p (Pull-up)
#
# PCU (PEx)
# 1. Port Group                : 4 (PCU Port E)
#
# 2. Port                      : [ 4 12 a 1 -pupd p ] [ 4 13 a 1 -pupd p ]
#    Pin Number                : 12 / 13
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 (Tx/Rx)
#    Pull-up/down              : p (Pull-up)
#
# PCU (PDx)
send ""
send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 2 a 1 -pupd p"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 3 a 1 -pupd p"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# PCU (PEx)
send "port 4 12 a 1 -pupd p"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 13 a 1 -pupd p"
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

send "config 2 i u 9 n 1 n 38400"
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

send "config 1 i u 9 n 1 n 38400"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

send "data 1 16 0x01 0xa5 0x00 0x5a 0x01 0xa5 0x00 0x5a 0x01 0x05 0x00 0x07 0x01 0x09 0x00 0xa0"
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
