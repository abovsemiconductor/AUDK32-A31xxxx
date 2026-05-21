# A. Description
#    A list of commands here configures RTC and generates a timestamp interrupt by external trigger signal (RTC_TA input).
#
# B. Preparation
#    Configure RTC_TA pin as a Timestamp trigger signal.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. RTC
#    2. PCU/GPIO
#
# D. Default Port
#    1. PCU     : PA0 (Output)
#    2. RTC0    : PC13 (RTC_TA)
#
# E. Port Connection
#    1. PA0 (Output) ----> PC13 (RTC_TA)
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
# 4. Clock(Time/Data)          : [ 0 22 5 4 3 1 1 50 m ] (2022/05/03 Thu PM 01:01:50)
#    Year                      : 2022
#    Month                     : 5
#    Week                      : 4
#    Day                       : 3
#    Hour                      : 1
#    Minute                    : 1
#    Second                    : 50
#    Event Period Mode         : m (Every 1 minute)
#
# 5. Timestamp                 : [ 0 e f ]
#    Mode Enable               : e (Enable)
#    Event Edge Mode           : f (Falling Edge)
#
# PCU (PA0)
# 1. Port Group                : 0 (PCU Port A)
#
# 2. Port                      : [ 0 0 o p h ]
#    Pin Number                : 0
#    Pin Mode                  : o (Output)
#    Pin Output Mode           : p (Push-Pull)
#    Pin Level                 : h (High)
#
# 3. Output                    : [ 0 0 l / h ]
#    Pin Number                : 0
#    Pin Level                 : l (Low) / h (High)
#
# PCU (PA0)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 0 o p h"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


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

send "clock 0 22 5 4 3 1 1 50 m"
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

send "timestamp 0 e f"
expect {
    "<RTC> # "
    break
    timeout 5 goto end
}


send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

set a 0
set b 10

loop:

inc a
send "output 0 0 l"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
sleep 1
send "output 0 0 h"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
sleep 1

if a < b goto loop

end:
