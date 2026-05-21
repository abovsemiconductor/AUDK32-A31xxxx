# A. Description
#    A list of commands here configures RTC and generates a real clock frequency at PC13 port.
#    The purpose of this configuration is only for measuring a real frequency of fRTC.
#
# B. Preparation
#    Configure RTC_TA pin as a RTC_OUT port.
#
# C. Default Port
#    1. RTC0 : PC13 (RTC_OUT)
#
# D. Note
#    Must multiply measured frequency by 2.
#
# For more information, read a user's manual of the target device carefully.
#
# RTC0
# 1. Channel                     : 0 (RTC 0)
#
# 2. Clock                       : [ 0 m l 10 ]
#    Source                      : m (MCCR : Misc Clock)
#    MCCR Source                 : l (LSI : Low Speed Internal Clock - 750KHz)
#    Source Divide               : 10 
#
# 3. Config                      : [ 0 i -meas ]
#    Operation                   : i (Interrupt)
#    Measuring fRTC at PC13 port : N/A
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

send "clk 0 m l 10"
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

end:
