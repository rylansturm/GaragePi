from gpiozero import LED
import sys
from time import sleep

DOORS = {
    1: 21,
    2: 20,
    3: 16,
    4: 12
}

def main(door, l):
    l.off()
    print(door, ' on.')
    sleep(.5)
    l.on()
    print(door, ' off.')


if __name__ == '__main__':
    try:
        door = int(sys.argv[1])
        door = DOORS[door]
        l = LED(door)
        main(door, l)
    except:
        raise ValueError("The argument passed to garageOpener.py was not an integer")
