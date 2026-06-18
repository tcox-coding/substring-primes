import csv

lines = ""

with open('primes1.txt', 'r') as f:
  lines = f.readlines()
  
# print(lines[:100])
all_primes = []

for line in lines[1:]:
  if line == '\n':
    continue
  
  line = line.strip()
  primes = [l for l in line.split(' ') if l != '']
  all_primes.extend(primes)


with open('all_primes_lt_1m.txt', 'w+') as f:
  f.writelines([prime + '\n' for prime in all_primes])