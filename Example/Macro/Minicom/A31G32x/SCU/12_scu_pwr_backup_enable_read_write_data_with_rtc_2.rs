# A. Description
#    A list of commands here configures SCU PWR and reads/writes backup data.
#
# B. Preparation
#    Supply power to external power port by a suitable instrument or make a environment to generate voltage over 1.6V.
#    Measure PC13 port whether proper frequency is available or not.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. SCUPWR
#    2. RTC
#
# D. Default Port
#    1. RTC0   : PC13 (RTC Clock Output)
#
# E. Note
#    RTC module should be enabled.
#
# For more information, read a user's manual of the target device carefully.
#
# SCUPWR
# 1. BAKDW                       : [ 0 0xa55aa55a 1 0xf0f0f0f0 2 0x5aa55aa5 3 0x0f0f0f0f ]
#    Backup Data Index           : 0 / 1 / 2 / 3
#    Backup Data                 : 0xa55aa55a / 0xf0f0f0f0 / 0x5aa55aa5 / 0x0f0f0f0f
#
# 2. BAKDR                       : [ 0 1 2 3 ]
#    Backup Data Index           : 0 / 1 / 2 / 3
#
# SCUPWR
send ""

send "cm scupwr"
expect {
    "<SCUPWR> # "
    break
    timeout 5 goto end
}

send "bakdr 0 1 2 3"
expect {
    "<SCUPWR> # "
    break
    timeout 5 goto end
}

send "bakdw 0 0xa55aa55a 1 0xf0f0f0f0 2 0x5aa55aa5 3 0x0f0f0f0f"
expect {
    "<SCUPWR> # "
    break
    timeout 5 goto end
}

send "bakdr 0 1 2 3"
expect {
    "<SCUPWR> # "
    break
    timeout 5 goto end
}

end:

