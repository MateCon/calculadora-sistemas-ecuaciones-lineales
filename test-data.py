import math
import random

string = ""
for _ in range(100):
    for _ in range(100): 
        string += str(math.floor(random.random() * 100)) + " "
    string += "\n"

print(string)