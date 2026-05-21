# A. Description
#    A list of commands here configures Temperature Sensor module and generates Average Sensor Clock count value
#    through interrupt by Temperature Sensor.
#
# B. Preparation
#    N/A
#
# C. Prerequisite Example (abov_example_config.h)
#    1. TEMP SENSOR
#
# For more information, read a user's manual of the target device carefully.
#
# 1. Channel                      : 0 (TempSens 0)
#
# 2. Config                       : [ 0 i e t 10000 ]
#    Operation                    : Interrupt
#    Reference Clock Source       : e (HSE : High Speed External Clock)
#    Sensor Clock Source          : t (LSITS : Low Speed Internal Temp Sensor Clock)
#    Reference Clock Match Count  : 10000
#
send ""

send "cm tempsens"
expect {
    "<TEMPSENS> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<TEMPSENS> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<TEMPSENS> # "
    break
    timeout 5 goto end
}

send "config 0 i e t 10000"
expect {
    "<TEMPSENS> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<TEMPSENS> # "
    break
    timeout 5 goto end
}

end:
