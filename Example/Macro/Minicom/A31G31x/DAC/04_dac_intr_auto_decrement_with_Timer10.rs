# A. Description
#    A list of commands here configures DAC and generates data reached interrupt.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. DAC
#    2. Timer1
#
# D. Default Port
#    1. DAC0    : PA6 (DAO)
#  
# For more information, read a user's manual of the target device carefully.
#
# DAC0
# 1. Config                     : [ 0 i a d i 0 -pg p 0 ]
#    Operation                  : i (Interrupt for Auto Mode)
#    Mode                       : a (Auto)
#    Direction for Auto Mode    : d (Decrement)
#    Reference Level            : i (Internal VDD)
#    Re-load reference          : 0 (Timer10)
#    Programmable Gain          : p (Positive)
#    Programmable Gain Level    : 0 (0dB)
#
# Timer10
# 1. Channel                   : 0 (Timer10)
#
# 2. Clock                     : [ 0 m h 10 1 ]
#    Source                    : m (MCCR : Misc Clock)
#    MCCR Source               : h (HSI : High Speed Internal Clock - 48MHz)
#    Source Divide             : 10 
#    Timer1 Pre-Divide         : 1 (MCCR Clock / 10) / 1
#
# 3. Config                    : [ 0 i p h 48 48 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Period)
#    Output Port Polarity      : h (High)
#    A Data                    : 48
#    B Data                    : 48
#
# Timer10
send ""

send "cm timer1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "clk 0 m h 10 1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "config 0 i p h 48 48"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "log off"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

# DAC0
send "cm dac"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "config 0 i a d i 0 -pg p 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

end:
