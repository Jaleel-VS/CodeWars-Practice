def filter_list(l):
    return [!(isinstance(item, str)) for item in l]

print(filter_list([1,2,'a','b']))