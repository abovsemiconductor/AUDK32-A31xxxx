# A. Description
#    A list of commands here configures USART as UART and generates receive time-out interrupt.
#
# B. Preparation
#    Connecting ports with a suitable instrument should be correct
#
# C. Prerequisite Example (abov_example_config.h)
#    1. USART
#
# D. Default Port
#    1. USART1  : PD2 (Transmit)
#               : PD3 (Receive)
#
# For more information, read a user's manual of the target device carefully.
#
# USART1
# 1. Channel                   : 1 (USART1)
#
# 2. Config                    : [ 1 i u 8 n 1 n 38400 -rto e 65535 ]
#    Operation                 : i (Interrupt)
#    USART Mode                : u (UART)
#    Data Bit                  : 8 (8bit)
#    Parity                    : n (None)
#    Stop Bit                  : 1 (1bit)
#    Speed Mode                : n (Normal)
#    Baudrate                  : 38400 bps
#    Receive Time Out Enable   : e (enable)
#    Receive Time Out Count    : 65535
#
# 3. Rx                        : [ 1 8 ]
#    Receive Data Length       : 8
#
# USART1
send ""

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

send "config 1 i u 8 n 1 n 38400 -rto e 65535"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

set a 0
set b 10

loop:
send "rx 1 8"
expect {
    "time-out !!!"
    break
    timeout 5 goto end
}
send ""
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

send "abort 1 r"
expect {
    "<USART> # "
    break
    timeout 5 goto end
}

inc a
if a < b goto loop

end:
