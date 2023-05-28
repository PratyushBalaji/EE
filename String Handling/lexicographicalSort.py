def sort(arr):
    ret = arr

    for i in range(len(ret)-1):
        for j in range(len(ret)-i-1):
            if ret[j] > ret[j+1]:
                temp = ret[j]
                ret[j] = ret[j+1]
                ret[j+1] = temp

    return ret

arr = ["hello","world","qux","foo","bar"]
print(sort(arr))