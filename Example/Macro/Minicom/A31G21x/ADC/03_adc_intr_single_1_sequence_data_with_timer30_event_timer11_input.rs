# A. Description
#    A list of commands here configures ADC and generates single capture interrupt.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
# 
# C. Prerequisite Example (abov_example_config.h)
#    1. ADC
#    2. Timer1
#    3. Timer3
#    4. PCU/GPIO
#
# D. Default Port
#    1. ADC0    : PA0 (AN0)
#               : PA1 (AN1)
#               : PA2 (AN2)
#    2. Timer11 : PE7 (Output/Capture Input)
#
# E. Port Connection
#    1. PA0 (AN0) <----> PE7 (Timer11 Output)
#
# F.Note 
#    1. Modify the line '#define __ROM_SIZE 0x00008000' to '#define __ROM_SIZE 0x00010000 in the ac5_a31g21x.sct file.
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
# 3. Config                     : [ config 0 i s i 1 3 0 ]
#    Operation                  : i (Interrupt)
#    Mode                       : s (Single)
#    Reference Level            : i (Internal VDD)
#    Sequence Count             : 1
#    Base Trigger Source        : 3 (Timer3)
#    Sampling Time              : 0 (No Sampling Time available so default 0)
#
# 4. Seq                        : [ 0 0 0 3 0 ]
#    Sequence Number            : 0
#    Input Channel              : 0
#    Trigger Source             : 3 (Timer3)
#    Trigger Source Number      : 0 (Timer30)
#
# Timer30
# 1. Channel                    : 0 (Timer30)
#
# 2. Clock                      : [ 0 m h 10 1000 ]
#    Source                     : m (MCCR : Misc Clock)
#    MCCR Source                : h (HSI : High Speed Internal Clock - 32MHz)
#    Source Divide              : 10
#    Timer1 Pre-Divide          : 1000 (MCCR Clock / 10) / 1000
#
# 3. Config                     : [ 0 i i s 6400 1600 3200 4800 ]
#    Operation                  : i (Interrupt)
#    Mode                       : i (Period/Interval)
#    Output Port Polarity       : s (6-ch)
#    Period Data                : 6400
#    A Data                     : 1600
#    B Data                     : 3200
#    C Data                     : 4800
#
# 4. Adctrg                     : [ 0 0x08 1 ]
#    Adc Trigger Number         : 0x08 (ADC trigger signal by Period Match)
#    Adc Trigger Delay          : 1 (Interrupt down count)
#
# Timer11
# 1. Channel                    : 1 (Timer11)
#
# 2. Clock                      : [ 0 m h 10 1000 ]
#    Source                     : m (MCCR : Misc Clock)
#    MCCR Source                : h (HSI : High Speed Internal Clock - 32MHz)
#    Source Divide              : 10
#    Timer1 Pre-Divide          : 1000 (MCCR Clock / 10) / 1000
#
# 3. Config                     : [ 0 i p h 6400 6400 ]
#    Operation                  : i (Interrupt)
#    Mode                       : p (Period)
#    Output Port Polarity       : h (High)
#    A Data                     : 6400
#    B Data                     : 6400
#
# PCU
# 1. Port Group                : 4 (PCU Port E)
#
# 2. Port                      : [ 4 7 a 1 ]
#    Pin Number                : 7
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 (Timer11 Output)
# 
# PCU
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 7 a 1"
expect {
    "<PCU> # "
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

send "clk 1 m h 10 1000"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "config 1 i p h 6400 6400"
expect {
    "<TIMER1> # "
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

send "log off"
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

send "config 0 i i s 6400 1600 3200 4800"
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

send "clk 0 p 63"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "config 0 i s i 1 3 0"
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

send "log on"
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


# Timer11
send "cm timer1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "start 1"
expect {
    "<TIMER1> # "
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

end:
