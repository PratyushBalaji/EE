def reverse(string):
    rev = ""
    for i in range(len(string)):
        index = len(string)-i-1
        rev += string[index]

    return rev


string = "hello"
print(reverse(string))
