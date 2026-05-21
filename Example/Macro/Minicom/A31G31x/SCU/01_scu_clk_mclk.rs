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
#    1. PCU    : PF4 (Clock Output)
#
# For more information, read a user's manual of the target device carefully.
#
# PCU (PF4)
# 1. Port                      : [ 5 4 a 1 ]
#    Port Group Number         : 5 (F group)
#    Pin Number                : 4
#    Pin Mode                  : Alternative (Mux)
#    Pin Alt Number            : 1
#
# SCUCLK
# 1. Clkout                    : [ m 0 e ]
#    Clock Source              : m (MCLK : Main Clock)
#    Clock Source Divide       : 0
#    Clock Output Enable       : e (Enable)
#
# 2. MCLK                      : [ e 0 ]
#    Clock Source              : e (HSE : High Speed External Clock)
#    Clock Source Divide       : 0
#
# PCU (PF4)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 5 4 a 1"
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

send "mclk e 0"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}

end:
