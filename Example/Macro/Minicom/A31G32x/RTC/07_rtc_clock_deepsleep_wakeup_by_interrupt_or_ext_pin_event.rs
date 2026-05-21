# A. Description
#    A list of commands here configures RTC and wakes up system by 1-minute period interrupt
#    or PC13 port external input.
#
# B. Preparation
#    Configure RTC_TA pin as a Timestamp trigger signal.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. RTC
#    2. SCUPWR
#
# D. Default Port
#    1. RTC0    : PC13 (TS_IN/RTC_OUT)
#
# For more information, read a user's manual of the target device carefully.
#
# RTC0
# 1. Channel                    : 0 (RTC 0)
#
# 2. Clock                      : [ 0 l ]
#    Source                     : l (LSI40KHz)
#
# 3. Config                    : [ 0 i ]
#    Operation                 : i (Interrupt)
#
# 4. Clock(Time/Data)           : [ 0 22 5 4 3 11 59 30 m ] (2022/05/03 Thu PM 11:59:30)
#    Year                       : 2022
#    Month                      : 5
#    Week                       : 4
#    Day                        : 3
#    Hour                       : 11
#    Minute                     : 59
#    Second                     : 30
#    Event Period Mode          : m (every 1 minute)
#
# 5. WKUP                       : [ 0 d e ]
#    Wakeup Source Enable       : e (Enable)
#    Wakeup by External Pin     : e (Enable)
# 
# 6. Timestamp                  : [ 0 e r ]
#    Mode Enable                : e (Enable)
#    Event Edge Mode            : r (Rising Edge)
#
# SCUPWR
# 1. AON                        : [ v:e b:e l:e ]
#    Always on Block Enable     : v (VDC) : e (enable) / b (BGR) : e (enable) / l (LSI) / e (enable)
#
# 2. PWR                        : [ d l 0 h 0 ]
#    Power Mode                 : d (Deep Sleep 1)
#    Sleep Clock Source         : l (LSI)
#    Sleep Clock Source Divide  : 0
#    Wakeup Clock Source        : h (HSI)
#    Wakeup Clock Source Divide : 0
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

send "config 0 i"
expect {
    "<RTC> # "
    break
    timeout 5 goto end
}

send "wkup 0 e e"
expect {
    "<RTC> # "
    break
    timeout 5 goto end
}

send "clock 0 22 5 4 3 11 59 30 m"
expect {
    "<RTC> # "
    break
    timeout 5 goto end
}

send "timestamp 0 e r"
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


# SCUPWR
send "cm scupwr"
expect {
    "<SCUPWR> # "
    break
    timeout 5 goto end
}

send "aon v:e b:e l:e"
expect {
    "<SCUPWR> # "
    break
    timeout 5 goto end
}

send "pwr d l 0 h 0"
expect {
    "<SCUPWR> # "
    break
    timeout 5 goto end
}

end:
