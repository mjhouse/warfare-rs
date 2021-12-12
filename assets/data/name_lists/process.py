#!/bin/env python

def process(input_path,output_path):
    with open(input_path) as inf:
        with open(output_path,'w') as outf:
            for line in inf.readlines():
                line = line.strip()
                if line:
                    line = line.capitalize()
                    outf.write(line + '\n')

if __name__=='__main__':
    process("last-names.txt","last_names.txt")