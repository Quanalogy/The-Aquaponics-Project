# The-Aquaponics-Project

This project aims to create an autonomous home made Aquaponics system.
In the long run we hope to be able to be able to run the system primarily from solar energy, with as many part of the project automated as possible.

# Design

The overall design document can be found in `Project.plantuml` and can be viewed with plantuml.

# Data collection

Currently the following sensors are to be implemented in the system (prioritized list):

1. Water flow sensor [YF-B6](https://www.seeedstudio.com/Water-Flow-Sensor-YF-B6-p-2883.html)
2. Water level sensor (TBD)
3. Water temperature sensor [PT100](https://www.ardu.dk/shop/pt100-temperaturfoeler-20-til-400-gr-c-vandtaet/)
4. Air temperature sensor (possible [DHT22](https://www.sparkfun.com/datasheets/Sensors/Temperature/DHT22.pdf))
5. Air humidity sensor (possible [DHT22](https://www.sparkfun.com/datasheets/Sensors/Temperature/DHT22.pdf))
6. Lux sensor (TBD)

The project is aim to be made with a BeagleBone Black Wireless for data collection and for providing a server on the home network, with an app to visualize the data.

# Code

## Building

### Building the container for cross compilation

A Dockerfile for cross compiling for `arm-unknown-linux-gnueabihf` can be found in `ContainerFiles/Dockerfile`. 
It be build with docker/podman with a terminal in the root of this project: `podman build -t rust-arm-builder ContainerFiles`.

### Cross-compiling 

You can either use cargo or what I would suggest to you instead; `cross`. 

Cross is essentially cargo for cross-compiling in containers, and it is setup such that it works with the build command just described. 
`cross` is installed by calling `cargo install cross`.

When cross is installed and the container is build, you can build the project with:

    cross build 
    
this will compile it for `arm-unknown-linux-gnueabihf`, if you want to change target you can do that with: 

    cross build --target arm-unknown-linux-gnueabihf

where `arm-unknown-linux-gnueabihf` must be replaced with the target you want to use.

If you want to use cargo instead, just replace cross with cargo.

## BeagleBone Black Wireless (BBBW)

As some of the sensors require ADC, the project will be targeting the BeagleBone Black, as it in opposition to the raspberry pi have build-in ADC.
It is a `arm-unknown-linux-gnueabihf` type device which is the default for this project. 


## Pi related

It is still a goal to make raspberry pi work with this project, and at some point maybe mainline the library such that it doesn't matter. 
Currently the focus will be on making the project work with the BeagleBone Black and first after it is stable will we look into supporting the pi.

Preliminary testing were done on raspberry pi 3 A (armv7) and raspberry pi zero w(armv6).
[The raspberry pi toolchain](https://github.com/raspberrypi/tools) will be used for linking.

## Water flow sensor (YF-B6)

The water flow sensor output the water flow rate (L/min) as per:

```
Q_flow [L/MIN]
F_out [Hz]
F_out = 6.6*Q_flow
Q_flow = F_out/6.6
```

## App related

The current idea is to implement in flutter, such that it is cross platform, but this have low priority in relation to the other parts.
