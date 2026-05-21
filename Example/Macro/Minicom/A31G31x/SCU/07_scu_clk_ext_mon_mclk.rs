# A. Description
#    A list of commands here configures SCU CLK and generates MCLK Clock Monitor Failure interrupt.
#
# B. Preparation
#    Set breakpoint by SW debugger.
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
# SCUCLK
# 1. MONITOR             : [ m e e m 3 ]
#    Clock Source        : m (MCLK : Main Clock)
#    Clock Recovery      : e (Disable)
#    Monitor Enable      : e (Enable)
#    Interrupt Mode      : m (Maskable)
#    Priority            : 3
#
# 2. Clkout              : [ m 0 e ]
#    Clock Source        : m (MCLK : Main Clock)
#    Clock Source Divide : 0
#    Clock Output Enable : e (Enable)
#
# 3. CLKEN               : [ h d ]
#    Clock Source        : h (HSI : High Speed Internal Clock)
#    Clock Enable        : d (Disable)
#
# PCU (PF4)
# 1. Port                : [ 5 4 a 1 ]
#    Port Group Number   : 5 (F group)
#    Pin Number          : 4
#    Pin Mode            : Alternative (Mux)
#    Pin Alt Number      : 1
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

send "monitor m e e m 3"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}
sleep 3
send "clken h d"

end:

