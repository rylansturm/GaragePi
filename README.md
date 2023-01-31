# GaragePi

I'm looking at smartifying my garage door. I've seen other projects,
and they look nice. It's more about making it myself than it is 
about having it in place.

## Specs

#### Basic Requirements

Non-destructive. The project must not remove anything from the current
garage door system. Wires can be added into and below the physical 
buttons that are on the garage wall now. Disconnection should be easy.

Open/Close. A relay can be used to make the connection that send the
signal to the garage openers. 

Scalable. The system should support my two garage doors, and
should ideally be scalable up or down easily. One door without useless
clutter in the UI, 3 doors just as simply.

Sensors. I should be able to read the current state of the garage.


#### MVP

A minimum viable project is one where I can control my garage door
via ssh when connected to WiFi. That just means Raspberry Pi, 
Relay board, and a local bash script. Let's go.
