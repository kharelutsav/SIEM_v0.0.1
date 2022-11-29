import string_sum
import time
import json
import re
import timeit

compile1 = re.compile(capture_regex_py)
compile2 = re.compile(find_regex_py)

first_run = time.time()
def run_python():
    match = compile1.search(capture_data)
    parsed_data = match.groupdict()
    tmp_val = compile2.findall(parsed_data.pop("attr"))
    normalized_data = {}
    for (k, v) in tmp_val:
        if k in _map:
            k = _map[k]
        normalized_data[k] = v
    for (k, v) in parsed_log.items():
        if k in _map:
            k = _map[k]
        normalized_data[k] = v
timeit.timeit(stmt='run_python()', setup='pass', number=10000, globals=globals())
print("\nNative python regex using re library time:", time.time()-first_run, '\n')

test_obj = string_sum.TestRegex(capture_regex_rs, find_regex)
second_run = time.time()
def run_rust():
    test_obj.normalize(capture_data)
    a = test_obj.normalized_data
timeit.timeit(stmt='run_rust()', setup='pass', number=10000, globals=globals())

print("\nPorted rust regex using regex library time:", time.time()-second_run, '\n')
