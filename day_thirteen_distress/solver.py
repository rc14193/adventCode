# The parsing with this for python was so easy I didn't want to do this day in rust
# The parsing in rust would have been an annoyance
import copy

input_file = "input.txt"

def compare(left, right):
    right = copy.deepcopy(right)
    left = copy.deepcopy(left)
    if left == [] and right == []:
        return 0
    elif left != [] and right == []:
        return -1
    elif left == [] and right != []:
        return 1
    else :
        if type(left[0]) == int and type(right[0]) == int:
            if left[0] < right[0]:
                return 1
            elif left[0] > right[0]:
                return -1
            else:
                return compare(left[1:], right[1:])
        elif type(left[0]) == int and type(right[0]) == list:
            res = compare([left[0]], right[0])
            if res == 0:
                return compare(left[1:], right[1:])
            else:
                return res
        elif type(left[0]) == list and type(right[0]) == int:
            right[0] = [right[0]]
            res = compare(left, right)
            if res == 0:
                return compare(left[1:], right[1:])
            else:
                return res
        elif type(left[0]) == list and type(right[0]) == list:
            res = compare(left[0],right[0])
            if res == 0:
                return compare(left[1:], right[1:])
            else:
                return res

with open(input_file, 'r') as f:
    lines = f.readlines()
    valid_lines = list(map(lambda y: eval(y),filter(lambda x: x != '\n', lines)))
    pairs = list(zip(valid_lines[::2],valid_lines[1::2]))
    vals = list(map(lambda x: compare(x[0], x[1]), pairs))
    #vals = compare(pairs[6][0], pairs[6][1])
    print(vals)
    for idx, val in enumerate(vals):
        if val == 1:
            vals[idx] = idx+1*val
        else:
            vals[idx] = 0
    print("Final val is", sum(vals))
    valid_lines.append([[2]])
    valid_lines.append([[6]])
    # bubble sort for quick and simple
    for i in range(len(valid_lines)-1):
        for j in range(len(valid_lines)-i-1):
            if compare(valid_lines[j],valid_lines[j+1]) == -1:
                valid_lines[j], valid_lines[j+1] = valid_lines[j+1], valid_lines[j]
    #print(vals)
    #print('\n'.join(map(lambda x: str(x), valid_lines)))
    two_loc = valid_lines.index([[2]])+1
    print("Two count is ", valid_lines.count([[2]]))
    six_loc = valid_lines.index([[6]])+1
    print("Six count is ", valid_lines.count([[6]]))
    print("[[2]] idx", two_loc)
    print("[[6]] idx", six_loc)
    print("decoder is", two_loc*six_loc)

    '''
'''