# Oak-Type
### Open Analog Keyboard

## Goal
Oak-Type's purpose is to be an analog-first firmware for keyboards and other computer
input devices, with an easily adjustable module system for defining new types of inputs
and outputs.

* Input and output types should be defined in a controller-agnostic way with traits and modules
* Implementations for specific hardware should require minimal code on top of a HAL
in conjunction with the oak-type library
* Configuring layouts should be able to be done through one or more config files which are
independent of the firmware
* Config files should be writable in any hierarchical format, such a JSON, XML, and YML
* Configuration files should be able to be loaded without requiring a reflash of the firmware,
ideally by saving the files to a mass-storage sector of keyboard memory
* The configured input and output types are mapped to one another in a tree data structure 

## Analog First
While oak-type should be able to handle digital inputs, the first first-class input type
is analog. Analog keyboard inputs enable features such as varying the input based on how
deep a key is pressed; differential inputs, such as sending a signal while the key is going
down, and stopping as soon as the key starts going up; and joystick-like inputs for steering
right from wasd.

## Firmware-independent configuration file
With config files that can be deployed after the firmware is flashed, QOL options start to
open up.
* Wireless keyboards don't have to be plugged in to be modified
* Less data needs to be transferred, speeding up iteration and reducing turnaround time
* A bad config file can't brick an otherwise functioning board

## Configuration layers in a tree structure
Layers are fantastic, and pass through keys that aren't configured on one layer functioning
the way they do on the next lower layer saves a lot of time writing config files, but leaves
a little to be desired.

In a tree structure, a board's root layer could be a typical qwerty, dvorak, or colemak layout.
The first branch could be games, disabling the meta key, with minor differences for what
game or genre you're playing. Another could be for coding modifying an underscore key to a
sticky shift when switching between a snake-case and camel-case language.

Another option could be for OS level differences. I hate reaching for the meta key when
working in one OS, when my brain tells me I need the cmd key. If they are the same button,
the brain doesn't have to care after the layer is changed
```
root
├─ games
│  ├─ shooter
│  ├─ rhythm
│  ├─ rts
├─ coding
│  ├─ Rust
│  ├─ Python
│  ├─ Java
├─ vim
```