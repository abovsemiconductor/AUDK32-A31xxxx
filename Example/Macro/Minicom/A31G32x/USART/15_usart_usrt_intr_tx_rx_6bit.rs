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
#    1. USART1  : PA4 (Serial Clock)
#               : PA2 (Transmit)
#               : PA3 (Receive)
#    2. USART2  : PB12 (Serial Clock)
#               : PB10 (Transmit)
#               : PB11 (Receive)
#
# E. Port Connection
#    1. PA4 (Serial Clock)  <----> PB12 (Serial Clock)
#    2. PA2 (Transmit)      <----> PB11 (Receive)
#    3. PA3 (Receive)       <----> PB10 (Transmit)
#
# For more information, read a user's manual of the target device carefully.
#
# USART1
# 1. Channel                   : 1 (USART1)
#
# 2. Config                    : [ 1 i u 7 n 1 n 38400 ]
#    Operation                 : i (Interrupt)
#    USART Mode                : u (UART)
#    Data Bit                  : 7 (7bit)
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
# 2. Config                    : [ 2 i u 7 n 1 n 38400 ]
#    Operation                 : i (Interrupt)
#    USART Mode                : u (UART)
#    Data Bit                  : 7 (7bit)
#    Parity                    : n (None)
#    Stop Bit                  : 1 (1bit)
#    Speed Mode                : n (Normal)
#    Baudrate                  : 38400 bps
#
# 3. Rx                        : [ 2 8 ]
#    Receive Data Length       : 8
#
# PCU (PAx)
# 1. Port Group                : 0 (PCU Port A)
#
# 2. Port                      : [ 0 4 a 2 -pupd p ] [ 0 2 a 0 -pupd p ] [ 0 3 a 0 -pupd p ]
#    Pin Number                : 4 / 2 / 3
#    Pin Mode                  : a (Alternative)
#    Alternative               : 2 / 0 / 0 (SCK/Tx/Rx)
#    Pull-up/down              : p (Pull-up)
#
# PCU (PBx)
# 1. Port Group                : 1 (PCU Port B)
# 
# 2. Port                      : [ 1 12 a 1 -pupd p ] [ 1 10 a 0 -pupd p ] [ 1 11 a 0 -pupd p ]
#    Pin Number                : 14 / 10 / 11
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 / 0 / 0 (SCK/Tx/Rx)
#    Pull-up/down              : p (Pull-up)
#
# PCU (PAx)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 4 a 2 -pupd p"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 2 a 0 -pupd p"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
send "port 0 3 a 0 -pupd p"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# PCU (PBx)
send "port 1 12 a 1 -pupd p"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 10 a 0 -pupd p"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 11 a 0 -pupd p"
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

send "config 2 i u 7 n 1 n 38400"
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

send "config 1 i u 7 n 1 n 38400"
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
