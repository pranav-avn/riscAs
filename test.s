# test risc-v assembly file
.main:
    addi t0, zero, 1      # initialize t0 to 1
    addi s0, zero, 0      # result (s0) = 0
    addi t1, zero, 10     # loop end value
.loop:
    add s0, s0, t0        # add to the result
    addi t0, t0, 1        # increment the counter
    bge t1, t0, . loop    # loop condition