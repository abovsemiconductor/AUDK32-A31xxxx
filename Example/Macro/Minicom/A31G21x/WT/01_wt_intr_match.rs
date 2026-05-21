# A. Description
#    A list of commands here configures WT module and generates a match interrupt.
#
# B. Preparation
#    N/A
#
# C. Prerequisite Example (abov_example_config.h)
#    1. WT
#
# Please, read the User Manual of the specific chip for more details.
#
# WT0
# 1. Channel                   : 0 (WT0)
#
# 2. Clock                     : [ 0 w ]
#    Source                    : w (WDTRC 31.250KHz)
#
# 3. Config                    : [ 0 i w 1 ]
#    Operation                 : i (Interrupt)
#    Divide                    : w (Clock / 2^14 * (Data + 1))
#    Data                      : 1
#
# WT0
send ""

send "cm wt"
expect {
    "<WT> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<WT> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<WT> # "
    break
    timeout 5 goto end
}

send "clk 0 w"
expect {
    "<WT> # "
    break
    timeout 5 goto end
}

send "config 0 i w 1"
expect {
    "<WT> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<WT> # "
    break
    timeout 5 goto end
}

end:
