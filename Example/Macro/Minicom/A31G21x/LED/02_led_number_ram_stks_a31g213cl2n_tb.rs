# A. Description
#    A list of commands here configure 7-segment(ram mode) and displays a digit on it.
#
# B. Preparation
#    N/A
#
# C. Prerequisite Example (abov_example_config.h)
#    1. LED
#
# D. Note
#    Should use UART1 as a debug log port at stks-a31g213cl2n_tb board.
#
# For more information, read a user's manual of the target device carefully.
#
# - 7-segment (Tested) : stks-a31g213cl2n_tb
#
# 1. Channel                    : 0 (LED)
#
# 2. Clock                      : [ 0 m h 1 64 ]
#    Source                     : MCCR (Misc Clock) / HSI (High Speed Internal Clock)
#    Source Divide              : 1
#    LED Pre-Divide             : 64
#
# 3. Config                     : [ 0 i p a 1 127 0 -overlap 32]
#    Operation                  : i (Interrupt)
#    Display Mode (RAM / Port)  : r (RAM)
#    Mode                       : a (Alone)
#    Port Share Mode            : 1 (iseq0-iseq1)
#    Pulse Width                : 127 ( Clock / (Width + 1) )
#    Stop Duration              : 0
#    Overlap                    : 32
#
# 4. Icom                       : [ 0 3 4 5 6 ]
#    Icom Number                : 3, 4, 5, 6
#
# 5. Iseg                       : [ 0 0 1 2 3 4 5 6 7 ]
#    Iseg Number                : 0, 1, 2, 3, 4, 5, 6, 7
#
# 6. Dim                        : [ 0 3 218 4 218 5 218 6 218 ]
#    Icom Number / Dimming      : 3 218, 4 218, 5 218, 6 218
#
# 7. Data                       : [ 0 3 0x3FF 4 0x3FF 5 0x3FF 6 0x3FF ]
#    Icom Number / ISeg Data    : 3 0x3FF, 4 0x3FF, 5 0x3FF, 6 0x3FF
#     - ISeg Data (Bit Order)
#
#FND Port configuration
#STKS_A31G213CL2N_TB 7-SEGMENT INFO
#- DIGIT 0: COM3
#- DIGIT 1: COM4
#- DIGIT 2: COM5
#- DIGIT 3: COM6
#- SEGMENT : SEG0 to SEG7
#      --(0)--
#     |       |
#    (5)      (1)
#     |--(6)--|
#    (4)      (2)
#     |       |
#      --(3)-- *(7)
#
#- 7-seg number
# [0:0x3F] [1:0x6] [2:0x5B] [3:0x4F] [4:0x66] [5:6D] [6:7D] [7:0x07] [8:0x7F] [9:0x6F]
#
send ""

send "cm led"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
send "uninit 0"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
send "init 0"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
send "clk 0 m h 1 64"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
send "config 0 i r a 1 128 0 -overlap 32"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
send "log off"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
send "icom 0 3 4 5 6"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
send "iseg 0 0 1 2 3 4 5 6 7"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
send "dim 0 3 218 4 218 5 218 6 218"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
# all display
send "data 0 3 0x3FF 4 0x3FF 5 0x3FF 6 0x3FF"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
send "start 0 200"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 2

# num 0 0 0 0
send "data 0 3 0x3F 4 0x3F 5 0x3F 6 0x3F"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
send "start 0 1000"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 6

# num 1 1 1 1
send "data 0 3 0x06 4 0x06 5 0x06 6 0x06"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
send "start 0 1000"
sleep 6

# num 2 2 2 2
send "data 0 3 0x5B 4 0x5B 5 0x5B 6 0x5B"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
send "start 0 1000"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 6

# num 3 3 3 3
send "data 0 3 0x4F 4 0x4F 5 0x4F 6 0x4F"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
send "start 0 1000"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 6

end:
