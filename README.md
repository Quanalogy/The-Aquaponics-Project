# The-Aquaponics-Project

This project aims to create an autonomous home made Aquaponics system.
In the long run we hope to be able to be able to run the system primarily from solar energy, with as many part of the project automated as possible.


## Design

The overall design document can be found in `Project.plantuml` and can be viewed with plantuml.

## Data collection

Currently the following sensors are to be implemented in the system (prioritized list):

1. Water flow sensor [YF-B6](https://www.seeedstudio.com/Water-Flow-Sensor-YF-B6-p-2883.html)
2. Water level sensor (TBD)
3. Water temperature sensor [PT100](https://www.ardu.dk/shop/pt100-temperaturfoeler-20-til-400-gr-c-vandtaet/)
4. Air temperature sensor (possible [DHT22](https://www.sparkfun.com/datasheets/Sensors/Temperature/DHT22.pdf))
5. Air humidity sensor (possible [DHT22](https://www.sparkfun.com/datasheets/Sensors/Temperature/DHT22.pdf))
6. Lux sensor (TBD)

The project is aim to be made with a raspberry pi for data collection and for providing a server on the home network, with an app to visualize the data. The pi does not have an ADC, hence this must be made possible through some chip, which is also TBD.


## Code

### Pi related

The data collection and server on the raspberry pi will be written in rust, tested on raspberry pi 3 A (armv7) and raspberry pi zero w(armv6).
[The raspberry pi toolchain](https://github.com/raspberrypi/tools) will be used for linking.

### App related

The current idea is to implement in flutter, such that it is cross platform, but this have low priority in relation to the other parts.
