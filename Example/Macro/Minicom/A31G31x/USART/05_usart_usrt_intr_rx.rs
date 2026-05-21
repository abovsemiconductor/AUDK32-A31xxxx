# A. Description
#    A list of commands here configures USART as USRT and receives data.
#
# B. Preparation
#    Connecting ports with a suitable instrument should be correct
#
# C. Prerequisite Example (abov_example_config.h)
#    1. USART
#    2. PCU/GPIO
#
# D. Default Port
#    1. USART1  : PD4 (Serial Clock)
#               : PD2 (Transmit)
#               : PD3 (Receive)
#
# For more information, read a user's manual of the target device carefully.
#
# USART1
# 1. Channel                   : 1 (USART1)
#
# 2. Config                    : [ 1 i a s 8 n 1 0 38400 ]
#    Operation                 : i (Interrupt)
#    USART Mode                : a (USRT)
#    USRT Mode                 : s (Slave)
#    Data Bit                  : 8 (8bit)
#    Parity                    : n (None)
#    Stop Bit                  : 1 (1bit)
#    Polarity                  : 0 (Tx Rising and Rx Falling Edge)
#    Baudrate                  : 38400 bps
#
# 3. Rx                        : [ 1 8 ]
#    Receive Data Length       : 8
#
# PCU (PDx)
# 1. Port Group                : 3 (PCU Port D)
#
# 2. Port                      : [ 3 4 a 2 -pupd p ] [ 3 2 a 1 -pupd p ] [ 3 3 a 1 -pupd p ]
#    Pin Number                : 4 / 2 / 3
#    Pin Mode                  : a (Alternative)
#    Alternative               : 2 / 1 / 1 (SCK/Tx/Rx)
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

send "port 3 4 a 2 -pupd p"
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

send "config 1 i a s 8 n 1 0 38400"
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
