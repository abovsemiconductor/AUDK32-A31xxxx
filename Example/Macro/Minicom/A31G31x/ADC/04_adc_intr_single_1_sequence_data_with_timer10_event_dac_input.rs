# A. Description
#    A list of commands here configures ADC and generates single capture interrupt by DAC output signal.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
# 
# C. Prerequisite Example (abov_example_config.h)
#    1. ADC
#    2. Timer1
#    3. DAC
#    4. PCU/GPIO
#
# D. Default Port
#    1. ADC0    : PA0 (AN0)
#               : PA1 (AN1)
#               : PA2 (AN2)
#    2. DAC0    : PA6 (DAO)
#
# E. Port Connection
#    1. PA6 (DAC0 DAO) ---> PA0 (AN0)
#
# For more information, read a user's manual of the target device carefully.
#
# ADC0
# 1. Channel                    : 0
#
# 2. Clock                      : [ 0 p 63 ]
#    Source                     : p (Periperhal Clock)
#    Peripheral Clock Divide    : 63 (Divide + 1)
#
# 3. Config                     : [ config 0 i s i 1 1 0 ]
#    Operation                  : i (Interrupt)
#    Mode                       : s (Single)
#    Reference Level            : i (Internal VDD)
#    Sequence Count             : 1
#    Base Trigger Source        : 1 (Timer1)
#    Sampling Time              : 0 (No Sampling Time available so default 0)
#
# 4. Seq                        : [ 0 0 0 1 0 ]
#    Sequence Number            : 0
#    Input Channel              : 0
#    Trigger Source             : 1 (Timer1)
#    Trigger Source Number      : 0 (Timer10)
#
# Timer10
# 1. Channel                    : 0 (Timer10)
#
# 2. Clock                      : [ 0 m h 10 1000 ]
#    Source                     : m (MCCR : Misc Clock)
#    MCCR Source                : h (HSI : High Speed Internal Clock - 48MHz)
#    Source Divide              : 10
#    Timer1 Pre-Divide          : 1000 (MCCR Clock / 10) / 1000
#
# 3. Config                     : [ 0 i p h 48 48 ]
#    Operation                  : i (Interrupt)
#    Mode                       : p (Period)
#    Output Port Polarity       : h (High)
#    A Data                     : 1200
#    B Data                     : 1200
#
# Timer11
# 1. Channel                    : 1 (Timer11)
#
# 2. Clock                      : [ 1 m h 10 10 ]
#    Source                     : m (MCCR : Misc Clock)
#    MCCR Source                : h (HSI : High Speed Internal Clock - 48MHz)
#    Source Divide              : 10
#    Timer1 Pre-Divide          : 10 (MCCR Clock / 10) / 10
#
# 3. Config                     : [ 1 i p h 120 120 ]
#    Operation                  : i (Interrupt)
#    Mode                       : p (Period)
#    Output Port Polarity       : h (High)
#    A Data                     : 120
#    B Data                     : 120
#
# DAC0
# 1. Config                     : [ 0 i a i i 1 -pg p 0 ]
#    Operation                  : i (Interrupt for Auto Mode)
#    Mode                       : a (Auto)
#    Direction for Auto Mode    : i (Increment)
#    Reference Level            : i (Internal VDD)
#    Re-load reference          : 1 (Timer11)
#    Programmable Gain          : p (Positive)
#    Programmable Gain Level    : 0 (0dB)
#
# PCU
# 1. Port Group                : 0 (PCU Port A)
#
# 2. Port                      : [ 0 6 a 3 ]
#    Pin Number                : 6
#    Pin Mode                  : a (Alternative)
#    Alternative               : 3 (DAC Output)
# 
# PCU
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 6 a 3"
expect {
    "<PCU> # "
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

send "log off"
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

send "config 0 i p h 1200 1200"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}


# Timer11
send "cm timer1"
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

send "clk 1 m h 10 10"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "config 1 i p h 120 120"
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

send "clk 0 p 63"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "config 0 i s i 1 1 0"
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

send "log on"
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

send "config 0 i a i i 1 -pg p 0"
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

end:

