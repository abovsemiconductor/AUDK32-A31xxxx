# A. Description
#    A list of commands here configures LCD module and displays a number on LCD.
#
# B. Preparation
#    Connecting ports with LCD should be correct.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. LCD
#
# D. Port Connection
#    1. PE0 (LCD COM0)   <----> COM0 (Port1)
#       PE1 (LCD COM1)   <----> COM1 (Port2)
#       PE2 (LCD COM2)   <----> COM2 (Port3)
#       PE3 (LCD COM3)   <----> COM3 (Port4)
#       PD5 (LCD SEG7)   <----> SEG7 (Port5)
#       PD4 (LCD SEG8)   <----> SEG8 (Port6)
#       PD3 (LCD SEG9)   <----> SEG9 (Port7)
#       PD2 (LCD SEG10)  <----> SEG10 (Port8)
#       PD1 (LCD SEG11)  <----> SEG11 (Port9)
#       PD0 (LCD SEG12)  <----> SEG12 (Port10)
#       PC12 (LCD SEG13) <----> SEG13 (Port11)
#       PC11 (LCD SEG14) <----> SEG14 (Port12)
#       PC10 (LCD SEG15) <----> SEG15 (Port13)
#
# For more information, read a user's manual of the target device carefully.
#
# - LCD Board (Tested) : GDC8310 TN LCD
#
# LCD0
# 1. Channel                    : 0
#
# 2. Clock                      : [ 0 m m 16 128 ]
#    Source                     : m (MCCR : Misc Clock)
#    MCCR Source                : m (MCLK : Main Clock)
#    MCCR Source Divide         : 16
#    LCD Pre-Divide             : 128
#
# 3. Config                     : [ 0 i 2 4 -auto 1 ]
#    Bias                       : i (Internal)
#    LCD Dividing Register      : 2 (66/66/50)
#    Duty                       : 4 (1/4 duty, 1/3 bias)
#    Timing by Auto Bias(-auto) : 1 (1 clock)
#
# 4. Data                       : [ 0 7 0x0F 8 0x0F 9 0x0F 10 0x0F 11 0x0F 12 0x0F 13 0x0F 14 0x0F 15 0x0F ]
#    Seg Number                 : 7, 8, 9, 10, 11, 12, 13, 14, 15
#    Com Bit                    : 0x0F (Com0, Com1, Com2, Com3)
#
# LCD0
send ""

send "cm lcd"
expect {
    "<LCD> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<LCD> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<LCD> # "
    break
    timeout 5 goto end
}

send "clk 0 m m 16 128"
expect {
    "<LCD> # "
    break
    timeout 5 goto end
}

send "config 0 i 2 4 -auto 1"
expect {
    "<LCD> # "
    break
    timeout 5 goto end
}

send "data 0 7 0x0F 8 0x0F 9 0x0F 10 0x0F 11 0x0F 12 0x0F 13 0x0F 14 0x0F 15 0x0F"
expect {
    "<LCD> # "
    break
    timeout 5 goto end
}

send "disp 0 on"
expect {
    "<LCD> # "
    break
    timeout 5 goto end
}
sleep 10

send "disp 0 off"
expect {
    "<LCD> # "
    break
    timeout 5 goto end
}

end:
