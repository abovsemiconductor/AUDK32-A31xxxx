# A. Description
#    A list of commands here configures ADC and generates multiple capture interrupt through timer1 trigger source.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. ADC
#    2. Timer1
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
# 2. Clock                     : [ 0 p 64 ]
#    Source                    : p (Peripheral Clock)
#    Peripheral Clock Divide   : 64
#
# 3. Config                    : [ config 0 i m i 8 3 0 ]
#    Operation                 : i (Interrupt)
#    Mode                      : m (Multiple)
#    Reference Level           : i (Internal VDD)
#    Sequence Count            : 8
#    Base Trigger Source       : 1 (Timer1)
#    Sampling Time             : 0
#
# 4. Seq                       : [ 0 0 0 1 0 ]
#    Sequence Number           : 0
#    Input Channel             : 0
#    Trigger Source            : 1 (Timer1)
#    Trigger Source Number     : 0 (Timer10)
#
# Timer1n
# 1. Channel                   : 0 / 1 / 2 (Timer1n)
#
# 2. Clock                     : [ 0 m h 10 1000 ] [ 1 m h 10 1000 ] [ 2 m h 10 1000 ]
#    Source                    : m (MCCR : Misc Clock)
#    MCCR Source               : h (HSI : High Speed Internal Clock - 48MHz)
#    Source Divide             : 10
#    Timer1 Pre-Divide         : 1000 (MCCR Clock / 10) / 1000
#
# 3. Config                    : [ 0 i p h 480 480 ] [ 1 i p h 960 960 ] [ 2 i p h 580 580 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Period)
#    Output Port Polarity      : h (High)
#    A Data                    : 480/960/580
#    B Data                    : 480/960/580
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

send "clk 0 m h 10 1000"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "config 0 i p h 480 480" 
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


# Timer11
send "uninit 1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "init 1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "clk 1 m h 10 1000"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "config 1 i p h 960 960" 
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}


# Timer12
send "uninit 2"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "init 2"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "clk 2 m h 10 1000"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "config 2 i p h 580 580" 
expect {
    "<TIMER1> # "
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

send "clk 0 p 3"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "config 0 i m i 8 1 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 0 0 1 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 1 1 1 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 2 2 1 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 3 3 1 1"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 4 4 1 1"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 5 5 1 1"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 6 6 1 2"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 7 7 1 2"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}


# Timer10
send "cm timer1"
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


# Timer11
send "start 1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}


# Timer12
send "start 2"
expect {
    "<TIMER1> # "
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

sleep 3 
send "dump 0"

end:
