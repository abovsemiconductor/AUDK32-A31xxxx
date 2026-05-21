# A. Description
#    A list of commands here configures DAC and generates Rx done dma interrupt by specific data.
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
# 1. Config                     : [ 0 d m i c -port 0x01 ]
#    Operation                  : d (Interrupt DMA)
#    Mode                       : m (Manual)
#    Reference Level            : i (Internal VDD)
#    Re-load reference          : c (Constant)
#    Output Port                : 1
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

send "config 0 d m i c -port 0x01"
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
