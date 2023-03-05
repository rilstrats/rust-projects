import random
import time

MIN=100
MAX=999
NUM_ITEMS=30000
l = [random.randint(MIN,MAX) for _ in range(NUM_ITEMS)]

# start at index 1 since item 0 is already "sorted"
start_time = time.time()
for unsort_i in range(1,len(l)):
    # store unsort_elm so it can inserted later
    unsort_elm = l[unsort_i] # necessary variable

    # print(f"for{unsort_i}: sta:  {l}")

    # set shift_i and shift_elm for first while iteration
    shift_i = unsort_i - 1 # necessary variable
    # shift_elm = l[shift_i]

    # while shift_i >= 0 and shift_elm > unsort_elm: 
    while shift_i >= 0 and l[shift_i] > unsort_elm: 
        # copy_i = shift_i + 1
        # l[copy_i] = shift_elm # copy shift_elm to the right
        l[shift_i+1] = l[shift_i] # copy shift_elm to the right

        # print(f"for{unsort_i}: whi{shift_i}: {l}: {shift_elm} copied")
        # print(f"for{unsort_i}: whi{shift_i}: {l}: {l[shift_i]} copied")

        # decrease shift_i and reset shift_elm for next while iteration
        shift_i -= 1
        # shift_elm = l[shift_i]

    
    # prev_shift_i = shift_i + 1
    # l[prev_shift_i] = unsort_elm # insert unsort_element into prev_shift_i
    l[shift_i+1] = unsort_elm # insert unsort_element into prev_shift_i

    # print(f"for{unsort_i}: end:  {l}: {unsort_elm} inserted\n")
print(time.time() - start_time)
