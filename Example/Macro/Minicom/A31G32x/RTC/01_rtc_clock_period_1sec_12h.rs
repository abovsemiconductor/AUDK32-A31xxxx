# A. Description
#    A list of commands here configures RTC and generates a clock period interrupt in 12h-system.
#
# B. Preparation
#    N/A
#
# C. Prerequisite Example (abov_example_config.h)
#    1. RTC
#
# For more information, read a user's manual of the target device carefully.
#
# RTC0
# 1. Channel                   : 0 (RTC 0)
#
# 2. Clock                     : [ 0 l ]
#    Source                    : l (LSI40KHz)
#
# 3. Config                    : [ 0 i ]
#    Operation                 : i (Interrupt)
#
# 4. Clock(Time/Data)          : [ 0 22 5 4 3 11 59 55 s ] (2022/05/03 Thu PM 11:59:55)
#    Year                      : 2022
#    Month                     : 5
#    Week                      : 4
#    Day                       : 3
#    Hour                      : 11
#    Minute                    : 59
#    Second                    : 55
#    Event Period Mode         : s (every 1 second)
#
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

send "clock 0 22 5 4 3 11 59 55 s"
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

end:
