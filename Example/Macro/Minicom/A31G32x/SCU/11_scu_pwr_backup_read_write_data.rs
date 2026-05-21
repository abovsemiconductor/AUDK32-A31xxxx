# A. Description
#    A list of commands here configures SCU PWR and reads/writes backup data.
#
# B. Preparation
#    N/A
#
# C. Prerequisite Example (abov_example_config.h)
#    1. SCUPWR
#    2. RTC
#
# D. Note
#    RTC module should be enabled.
#
# For more information, read a user's manual of the target device carefully.
#
# SCUPWR
# 1. BAKDW                       : [ 0 0xffff0000 1 0xaaaa5555 2 0x5555aaaa 3 0x0000ffff ]
#    Backup Data Index           : 0 / 1 / 2 / 3
#    Backup Data                 : 0xffff0000 / 0xaaaa5555 / 0x5555aaaa / 0x0000ffff
#
# 2. BAKDR                       : [ 0 1 2 3 ]
#    Backup Data Index           : 0 / 1 / 2 / 3
#
# RTC0
# 1. Channel                     : 0 (RTC 0)
#
# RTC0
send ""

send "cm rtc"
expect {
    "<RTC> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<RTC> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<RTC> # "
    break
    timeout 5 goto end
}


# SCUPWR
send "cm scupwr"
expect {
    "<SCUPWR> # "
    break
    timeout 5 goto end
}

send "bakdw 0 0xffff0000 1 0xaaaa5555 2 0x5555aaaa 3 0x0000ffff"
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
