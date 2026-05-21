# A. Description
#    A list of commands here configures SCU PWR and reads/writes backup data.
#
# B. Preparation
#    Measure PC13 port whether proper frequency is available or not.
#
# For more information, read a user's manual of the target device carefully.
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
# SCUPWR
# 1. BAKDR                       : [ 0 1 2 3 ]
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

end:

