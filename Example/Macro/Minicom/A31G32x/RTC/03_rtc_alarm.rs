# A. Description
#    A list of commands here configures RTC and generates a alarm interrupt.
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
# 2. Clock                     : [ 0 m l 6 ]
#    Source                    : m (MCCR : Misc Clock)
#    MCCR Source               : l (LSI : Low Speed Internal Clock - 750KHz)
#    Source Divide             : 6 
#
# 3. Config                    : [ 0 i ]
#    Operation                 : i (Interrupt)
#
# 4. Clock(Time/Data)          : [ 0 22 5 4 3 1 1 50 s ] (2022/05/03 Thu 01:01:50)
#    Year                      : 2022
#    Month                     : 5
#    Week                      : 4
#    Day                       : 3
#    Hour                      : 1
#    Minute                    : 1
#    Second                    : 50
#    Event Period Mode         : s (Every 1 second)
#
# 5. Alarm                     : [ 0 e s:w:h:r 1 2 ]
#    Mode Enable               : e (Enable)
#    Week                      : s:w:h:r (sun:wed:thu:sat)
#    Hour                      : 1
#    Minute                    : 2
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

send "clk 0 m l 6"
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

send "clock 0 22 5 4 3 1 1 50 s"
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

send "alarm 0 e s:w:h:r 1 2"
expect {
    "<RTC> # "
    break
    timeout 5 goto end
}

end:

