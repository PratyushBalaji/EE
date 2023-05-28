def search(substring,string):
    for i in range(len(string)-len(substring)+1):
        if string[i:i+len(substring)] == substring:
            return True
    return False

string = "hello"
substring = "llo"
print(search(substring,string))