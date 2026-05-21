# A. Description
#    A list of commands here configures ADC and generates single capture and comparator matched interrupt by DAC output signal.
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
#    2. DAC0    : PA4 (DAO)
#
# E. Port Connection
#    1. PA4 (DAC0 DAO) ---> PA0 (AN0)
#
# For more information, read a user's manual of the target device carefully.
#
# ADC0
# 1. Channel                    : 0
#
# 2. Clock                      : [ 0 p 64 ]
#    Source                     : p (Peripheral Clock)
#    Peripheral Clock Divide    : 64
#
# 3. Config                     : [ config 0 i s i 1 1 0 ]
#    Operation                  : i (Interrupt)
#    Mode                       : s (Single)
#    Reference Level            : i (Internal VDD)
#    Sequence Count             : 1
#    Base Trigger Source        : 1 (Timer1)
#    Sampling Time              : 0
#
# 4. Seq                        : [ 0 0 0 1 0 ]
#    Sequence Number            : 0
#    Input Channel              : 0
#    Trigger Source             : 1 (Timer1)
#    Trigger Source Number      : 0 (Timer10)
#
# 5. Cmp                        : [ 0 e 0 3072 -intr e d ]
#    Comparator Enable          : e (Enable)
#    Input Channel              : 0 
#    Comparator Data            : 3072
#    Interrupt Enable           : e (Enable)
#    Interrupt Trigger          : d (Input Channel Sampling Data <= Comparator Data)
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
# 3. Config                     : [ 0 i p h 2400 2400 ]
#    Operation                  : i (Interrupt)
#    Mode                       : p (Period)
#    Output Port Polarity       : h (High)
#    A Data                     : 2400
#    B Data                     : 2400
#
# DAC0
# 1. Config                     : [ 0 p m ign i c -port 0x01 -pg p 0]
#    Operation                  : p (Polling)
#    Mode                       : m (Manual)
#    Internal Test Procedure    : ign (Ignore)
#    Reference Level            : i (Internal VDD)
#    Re-load reference          : c (Constant)
#    Output Port                : 0x01 (DAC_OUT1)
#    Programmable Gain          : p (Positive)
#    Programmable Gain Level    : 0 (0dB)
#
# PCU (PA4)
# 1. Port Group                : 0 (PCU Port A)
#
# 2. Port                      : [ 0 4 a 10 ]
#    Pin Number                : 4
#    Pin Mode                  : a (Alternative)
#    Alternative               : 10 (DAC_OUT1)
# 
# PCU (PA4)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 4 a 10"
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

send "config 0 i p h 2400 2400"
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

send "clk 0 p 64"
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

send "cmp 0 e 0 3072 -intr e d"
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

send "config 0 p m ign i c -port 0x01 -pg p 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "data 0 0x0"
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

# DAC0
call ./08_adc_intr_single_1_sequence_data_compare_up_with_timer10_event_dac_input_1.rs

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


# Timer10 
send "cm timer1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "stop 0"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

end:

