#!/usr/bin/env python3

from os import X_OK
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

def find_orbit(orbits, orbit, target, parents):
    #print(orbit.name, orbit.children, parents)
    if target in orbit.children:
        print("!!!", parents)
        return parents + [orbit.name]

    match = None
    for child_name in orbit.children:
        child = orbits[child_name]
        x = find_orbit(orbits, child, target, parents + [orbit.name])
        if x is not None:
            match = x
    return match

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
    p1 = find_orbit(orbits, orbits['COM'], 'YOU', [])
    p2 = find_orbit(orbits, orbits['COM'], 'SAN', [])

    common = 0
    for p in p1:
        if p2[common] != p:
            break
        else:
            print(p, p2[common])
            common += 1

    path = p1[common:] + p2[common:]
    print(path)
    print(len(path))

    print(len(p1) - common, len(p2) - common, (len(p1) - common) + (len(p2) - common))

if __name__ == "__main__":
    main()

