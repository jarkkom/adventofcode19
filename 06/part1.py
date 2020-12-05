#!/usr/bin/env python3

import sys
from pprint import pprint

class Orbit:

    def __init__(self, name):
        self.name = name
        self.children = []

total = 0

def print_orbit(orbits, orbit, count):
    print(orbit.name, orbit.children)
    for child_name in orbit.children:
        child = orbits[child_name]
        global total
        total = total + count
        print("%s)%s %d" % (orbit.name, child.name, count))
        print_orbit(orbits, child, count + 1)

    return count

def main():
    file = open(sys.argv[1])

    orbits = dict()

    lines = file.readlines()
    for line in lines:
        (parent_name, child_name) = line.strip().split(')')
        print(parent_name, child_name)

        child = None
        if child_name in orbits:
            child = orbits[child_name]
        else:
            child = Orbit(child_name)
            orbits[child_name] = child

        parent = None

        if parent_name in orbits:
            parent = orbits[parent_name]
        else:
            parent = Orbit(parent_name)
            orbits[parent_name] = parent

        parent.children.append(child_name)

    print("-----")
    print_orbit(orbits, orbits['COM'], 1)

    print(total)

if __name__ == "__main__":
    main()

