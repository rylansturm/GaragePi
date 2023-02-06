from gpiozero import LED
import sys
from time import sleep

DOORS = {
    1: 21,
    2: 20,
    3: 26,
    4: 16
}

def main(door):
    l = LED(door)
    l.on()
    print(door, ' on.')
    sleep(.5)
    l.off()
    print(door, ' off.')


if __name__ == '__main__':
    try:
        door = int(sys.argv[1])
        door = DOORS[door]
        main(door)
    except:
        raise ValueError("The argument passed to garageOpener.py was not an integer")
