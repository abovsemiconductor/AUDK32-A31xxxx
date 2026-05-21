# A. Description
#    A list of commands here configures SCU PWR and backup power mode.
#
# B. Preparation
#    Supply power to external power port by a suitable instrument or make a environment to generate low voltage under 1.6V. 
#    Measure PC13 port whether proper frequency is available or not.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. SCUPWR
#    2. SCULVD
#    3. RTC
#
# D. Default Port
#    1. RTC0   : PC13 (RTC Clock Output)
#
# For more information, read a user's manual of the target device carefully.
#
# SCUPWR
# 1. BAK                         : [ e ]
#    Backup Mode                 : e (enable)
#
# SCULVR
# 1. LVR                         : [ d ]
#    Enable                      : d (disable)
#
# RTC0
# 1. Channel                     : 0 (RTC 0)
#
# 2. Clock                       : [ 0 l ]
#    Source                      : l (LSI40KHz)
#
# 3. Config                      : [ 0 i -meas ]
#    Operation                   : i (Interrupt)
#    Measuring fRTC at PC13 port : N/A
#
# RTC0
send ""

send "cm rtc"
expect {
    "<RTC> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<RTC> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<RTC> # "
    break
    timeout 5 goto end
}

send "clk 0 l"
expect {
    "<RTC> # "
    break
    timeout 5 goto end
}

send "config 0 i -meas"
expect {
    "<RTC> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<RTC> # "
    break
    timeout 5 goto end
}


# SCULVD
send "cm sculvd"
expect {
    "<SCULVD> # "
    break
    timeout 5 goto end
}

send "lvr d"
expect {
    "<SCULVD> # "
    break
    timeout 5 goto end
}

# SCUPWR
send "cm scupwr"
expect {
    "<SCUPWR> # "
    break
    timeout 5 goto end
}

send "bak e"
expect {
    "<SCUPWR> # "
    break
    timeout 5 goto end
}

end:
