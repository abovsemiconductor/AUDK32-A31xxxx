# A. Description
#    A list of commands here configures DAC and generates data reached interrupt.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. DAC
# 
# D. Default Port
#    1. DAC0    : PA6 (DAO)
#  
# For more information, read a user's manual of the target device carefully.
#
# DAC0
# 1. Config                     : [ 0 i a i i c -pg p 0 ]
#    Operation                  : i (Interrupt for Auto Mode)
#    Mode                       : a (Auto)
#    Direction for Auto Mode    : i (Increment)
#    Reference Level            : i (Internal VDD)
#    Re-load reference          : c (Constant)
#    Programmable Gain          : p (Positive)
#    Programmable Gain Level    : 0 (0dB)
#
# DAC0
send ""

send "cm dac"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "config 0 i a i i c -pg p 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

end:
