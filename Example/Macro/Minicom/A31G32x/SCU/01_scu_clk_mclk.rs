# A. Description
#    A list of commands here configures SCU CLK and generates a certain frequency of Main Clock.
#
# B. Preparation
#    Make an environment to measure clock frequency by clock output port.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. SCUCLK
#    2. PCU/GPIO
#
# D. Default Port
#    1. PCU    : PF0 (Clock Output)
#
# For more information, read a user's manual of the target device carefully.
#
# PCU (PF0)
# 1. Port                      : [ 5 0 a 1 ]
#    Port Group Number         : 5 (F group)
#    Pin Number                : 0
#    Pin Mode                  : Alternative (Mux)
#    Pin Alt Number            : 1
#
# SCUCLK
# 1. Clkout                    : [ m 0 e ]
#    Clock Source              : m (MCLK : Main Clock)
#    Clock Source Divide       : 0
#    Clock Output Enable       : e (Enable)
#
# 2. MCLK                      : [ l 0 ]
#    Clock Source              : l (LSI : Low Speed Internal Clock)
#    Clock Source Divide       : 0
#
# PCU (PF0)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 5 0 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# SCUCLK
send "cm scuclk"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}

send "clkout m 0 e"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}

send "mclk l 0"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}

end:
