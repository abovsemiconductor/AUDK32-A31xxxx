# A. Description
#    A list of commands here configures PCU module and controls a output level of a target port.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. PCU/GPIO
#
# D. Default Port
#    1. PCU     : PC13 (Output)
#               : PC14 (Output)
#               : PC15 (Output)
#               : PA0 (Input)
#               : PA1 (Input)
#               : PA2 (Input)
#
# E. Port Connection
#    1. PC13 (Output) ----> PA0 (Input)
#       PC14 (Output) ----> PA1 (Input)
#       PC15 (Output) ----> PA2 (Input)
#
# For more information, read a user's manual of the target device carefully.
#
# PCU (PCx)
# 1. Port Group                : 2 (PCU Port C)
#
# 2. Port                      : [ 2 13 o p h ] [ 2 14 o p h ] [ 2 15 o p h ]
#    Pin Number                : 13 / 14 / 15
#    Pin Mode                  : o (Output)
#    Pin Output Mode           : p (Push-Pull) 
#    Pin Level                 : h (High)
#
# 3. Output                    : [ 2 13 l / h ] [ 2 14 l / h ] [ 2 15 l / h ]
#    Pin Number                : 13 / 14 / 15
#    Pin Level                 : l (Low) / h (High)
#
# PCU (PAx)
# 1. Port Group                : 0 (PCU Port A)
#
# 2. Port                      : [ 0 0 i i ] [ 0 1 i i ] [ 0 2 i i ]
#    Pin Number                : 0 / 1 / 2
#    Pin Mode                  : i (input)
#    Pin Output Mode           : i (input)
#
# 3. Input                     : [ 0 0 ] [ 0 1 i i ] [ 0 2 i i ]
#    Pin Number                : 0
#
# PCU (PC13)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 2 13 o p h"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# PCU (PC14)
send "port 2 14 o p h"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# PCU (PC15)
send "port 2 15 o p h"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# PCU (PA0)
send "port 0 0 i i"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# PCU (PA1)
send "port 0 1 i i"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# PCU (PA2)
send "port 0 2 i i"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

set a 0
set b 15

loop:
inc a
send "output 2 13 l"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
send "output 2 14 h"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
send "output 2 15 l"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
sleep 0.3
send "input 0 0"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
send "input 0 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
send "input 0 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
sleep 1
        
send "output 2 13 h"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
        
send "output 2 14 l"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
        
send "output 2 15 h"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
sleep 0.3        
send "input 0 0"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
send "input 0 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
send "input 0 2"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
sleep 1

if a < b goto loop

end:
