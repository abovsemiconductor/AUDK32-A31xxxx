# A. Description
#    A list of commands here configures CMP and generates both edge detected interrupt.
#
# B. Preparation
#    Supply reference 1V and source signal to specific port by a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. CMP
#    2. DAC
#
# D. Default Port
#    1. CMP0    : PA1 (CP0_PO)
#               : PA0 (CP0_N2)
#    2. DAC0    : PA4 (DACO)
#
# E. Port Connection
#    1. Supply Power + ----> PA0 (CP0_N2)
#       Supply Power - ----> GND
#       DACO (PA4) -----> PA1 (CP0_P0)
#
# For more information, read a user's manual of the target device carefully.
#
# CMP0
# 1. Config                     : [ 0 i m 0 0 h b -dbc 128 8 d ]
#    Operation                  : i (Interrupt)
#    Interrupt Mode             : m (Maskable)
#    Source                     : 0 (CP0_PO)
#    Reference                  : 0 (CP0_N2)
#    Hysteresis                 : h (high)
#    Trigger Mode               : b (Both Edge)
#    Debounce (-dbc)
#    Clock Divide               : 128 (Peripheral Clock/(Clock Divide*2))
#    Shift Clock                : 8
#    LSI Enable                 : d (Disable)
#
# CMP0
send ""

send "cm cmp"
expect {
    "<CMP> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<CMP> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<CMP> # "
    break
    timeout 5 goto end
}

send "config 0 i m 0 0 h b -dbc 128 8 d"
expect {
    "<CMP> # "
    break
    timeout 5 goto end
}

send "log on 1"
expect {
    "<CMP> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<CMP> # "
    break
    timeout 5 goto end
}

call ../DAC/01_dac_polling_manual_signal_out.rs

end:
