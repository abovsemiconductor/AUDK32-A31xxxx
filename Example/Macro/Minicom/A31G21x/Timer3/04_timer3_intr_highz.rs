# A. Description
#    A list of commands here configures timer3 and generates a high-z and a interval interrupt.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Timer3
#    2. Timer2
#
# D. Default Port
#    1. Timer30 : PE0 (PWM AA Output)
#               : PE1 (PWM AB Output)
#               : PE2 (PWM BA Output)
#               : PE3 (PWM BB Output)
#               : PE4 (PWM CA Output)
#               : PE5 (PWM CB Output)
#               : PF7 (Capture Input)
#               : PF2 (External Clock Input)
#               : PF5 (BLANK Input)
#    2. Timer20 : PC0/PC0 (Output/Capture Input)
#               : PC2 (External Clock Input)
#
# E. Port Connection
#    1. PC0 (Timer20 Output) ----> PF5 (Timer30 BLANK Input)
#
# For more information, read a user's manual of the target device carefully.
#
# Timer30
# 1. Channel                   : 0 (Timer30)
#
# 2. Clock                     : [ 0 m h 10 1000 ]
#    Source                    : m (MCCR : Misc Clock)
#    MCCR Source               : h (HSI : High Speed Internal Clock - 32MHz)
#    Source Divide             : 10 
#    Timer3 Pre-Divide         : 1000 (MCCR Clock / 10) / 1000
#
# 3. Config                    : [ 0 i i s -hiz r e m 12800 3200 6400 9600 ]
#    Operation                 : i (Interrupt)
#    Mode                      : i (Interval)
#    Capture Edge              : b (Both Edge)
#    Output Mode               : s (6-ch)
#    High-Z Edge               : r (Rising)
#    High-Z Source             : e (External Blank)
#    High-Z Interrupt Mode     : m (Maskable)
#    Period                    : 12800
#    A Data                    : 3200
#    B Data                    : 6400
#    C Data                    : 9600
#
# Timer20 
# 1. Channel                   : 0 (Timer20)
#
# 2. Clock                     : [ 0 m h 10 1000 ]
#    Source                    : m (MCCR : Misc Clock)
#    MCCR Source               : h (HSI : High Speed Internal Clock - 32MHz)
#    Source Divide             : 10 
#    Timer2 Pre-Divide         : 1000 (MCCR Clock / 10) / 1000
#
# 3. Config                    : [ 0 i p h 3200 3200 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Period)
#    Output Port Polarity      : h (High)
#    A Data                    : 3200
#    B Data                    : 3200
#
# Timer20
send ""

send "cm timer2"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "clk 0 m h 10 1000"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "config 0 i p h 3200 3200"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "log off"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}


# Timer30 
send "cm timer3"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

send "clk 0 m h 10 1000"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

send "config 0 i i s -hiz r e m 12800 3200 6400 9600"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

send "log on"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

end:
