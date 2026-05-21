# A. Description
#    A list of commands here configures CFMC and erase and write code flash
#
# B. Preparation
#    Setup ARM debugger to connect device without reset to see registers and flash memory
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Flash
#
# For more information, read a user's manual of the target device carefully.
#
# 1. Code Flash Size : 0x8000(MP)/0x10000(MP)
#    0x8000 is taken as flash size to test
# 2. Code Write Protection Size  : 0x800
# 3. Erase Mode
#    - CFMC_CHIP_ERASE_MODE = 0
#    - CFMC_PAGE_ERASE_MODE = 1
#    - CFMC_1KB_ERASE_MODE  = 2
#    - CFMC_2KB_ERASE_MODE  = 4
#    - CFMC_4KB_ERASE_MODE  = 8
#    CFMC_1KB_ERASE_MODE is taken to test code flash.

######################  Define Variables ####################### 
# CFMC_CHIP_ERASE_MODE = 0
# CFMC_PAGE_ERASE_MODE = 1
# CFMC_1KB_ERASE_MODE  = 2
# CFMC_2KB_ERASE_MODE  = 4
# CFMC_4KB_ERASE_MODE  = 8

# FLASH_SIZE = 32768

# SIZE_2KB  = 2048
# SIZE_1KB  = 1024
# SIZE_512B = 512

# START_TEST_ADDR = FLASH_SIZE - SIZE_1KB

# DUMP_ADDR = START_TEST_ADDR
# DUMP_LEN = SIZE_1KB
# DISP_MESSAGE = ""

######################  Test commands here ####################### 
send ""

send "cm cfmc"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}

# Print flash geometric information
print "01.Print flash geometric information"
send ""
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "info"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
sleep 2

# Unprotect all code flash area
print "\n02.Unprotect all code flash area"
send ""
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "wprot 0xFFFFFFFF 0 0"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
sleep 2

# Erase 1KB at FLASH_SIZE
print "\n03.Erase 1KB at 0x7C00"
send ""
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "erase 0x7C00 2"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
sleep 2

print "\n04.Dump memory 0x7C00 ~ 0x7FFF)"
send "cm scu"
send "rmem 0x7C00 256" 
expect {
    "<SCU> #"
    break
    timeout 30 goto end
}
sleep 2

# Write predefined 32B data (0x00 ~ 0x1F) at START_TEST_ADDR into flash write protected area
# See the result of 'info' command
print "\n05.Write 32Byte data (0x00 ~ 0x1F) to flash write protected area 0x7C00 ~ 0x7FFF)" 
send ""
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "cm cfmc"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "write 0x7C00 0xFFFFFFFF"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
sleep 2

# Read 1KB
print "\n06.Dump memory (0x7C00 ~ 0x7FFF)"
send ""
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "cm scu"
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "rmem 0x7C00 256" 
expect {
    "<SCU> # "
    break
    timeout 30 goto end
}
sleep 2

# Self Erase (512B) write protect sector at START_TEST_ADDR
print "\n07.Self Erase per 512B (0x7C00 ~ 0x7FFF)"
send ""
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "cm cfmc"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}

send "selferase 0x7C00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x7D00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x7E00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x7F00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}
sleep 2

# Read 1KB
print "\n08.Dump memory (0x7C00 ~ 0x7FFF)"
send ""
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "cm scu"
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "rmem 0x7C00 256" 
expect {
    "<SCU> # "
    break
    timeout 30 goto end
}
sleep 2

# Self-Write predefined 32B data (0x00 ~ 0x1F) at START_TEST_ADDR into flash write protected area
print "\n09.Self Write 32Byte data (0x00 ~ 0x1F) to flash write protected area (0x7C00 ~ 0x7FFF)"
send ""
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "cm cfmc"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "selfwrite 0x7C00 0xFFFFFFFF"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
sleep 2

# Read 1KB
print "\n10.Dump memory (0x7C00 ~ 0x7FFF)"
send ""
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "cm scu"
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "rmem 0x7C00 256" 
expect {
    "<SCU> # "
    break
    timeout 30 goto end
}

print "\nEnd of Flash Test Sequence"

end:

