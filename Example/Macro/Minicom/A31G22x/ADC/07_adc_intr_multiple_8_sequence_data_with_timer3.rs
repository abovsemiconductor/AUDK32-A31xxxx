# A. Description
#    A list of commands here configures ADC and generates multiple capture interrupt through timer3 trigger source.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. ADC
#    2. Timer3
#
# D. Default Port
#    1. ADC0    : PA0 (AN0)
#               : PA1 (AN1)
#               : PA2 (AN2)
#
# For more information, read a user's manual of the target device carefully.
#
# ADC0
# 1. Channel                   : 0
#
# 2. Clock                     : [ 0 m h 128 ]
#    Source                    : m (MCCR : Misc Clock)
#    MCCR Source               : h (HSI : High Speed Internal Clock - 32MHz)
#    MCCR Source Divide        : 128
#
# 3. Config                    : [ config 0 i m i 8 3 0 ]
#    Operation                 : i (Interrupt)
#    Mode                      : m (Multiple)
#    Reference Level           : i (Internal VDD)
#    Sequence Count            : 8
#    Base Trigger Source       : 3 (Timer3)
#    Sampling Time             : 0
#
# 4. Seq                       : [ 0 0 0 3 0 ]
#    Sequence Number           : 0
#    Input Channel             : 0
#    Trigger Source            : 3 (Timer3)
#    Trigger Source Number     : 0 (Timer30)
#
# Timer30
# 1. Channel                   : 0 (Timer30)
#
# 2. Clock                     : [ 0 m h 10 1000 ]
#    Source                    : m (MCCR : Misc Clock)
#    MCCR Source               : h (HSI : High Speed Internal Clock - 32MHz)
#    Source Divide             : 10
#    Timer1 Pre-Divide         : 1000 (MCCR Clock / 10) / 1000
#
# 3. Config                    : [ 0 i i s 3200 800 1600 2400 ]
#    Operation                 : Interrupt
#    Mode                      : Period
#    Output Port Polarity      : High Level
#    Period Data               : 3200
#    A Data                    : 800
#    B Data                    : 1600
#    C Data                    : 2400
#
# 4. Adctrg                    : [ 0 0x08 1 ]
#    Adc Trigger Number        : 0x08 (ADC trigger signal by Period Match)
#    Adc Trigger Delay         : 3 (Interrupt down count)
#
# Timer30
send ""

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

send "config 0 i i s 3200 800 1200 2400"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

send "adctrg 0 0x08 1"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}


# ADC0
send "cm adc"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "clk 0 m h 128"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "config 0 i m i 8 3 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 0 0 3 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 1 1 3 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 2 2 3 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 3 3 3 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 4 4 3 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 5 5 3 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 6 6 3 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 7 7 3 0"
expect {
    "<ADC> # "
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

send "start 0"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}


# ADC0
send "cm adc"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

sleep 12
send "dump 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

end:
