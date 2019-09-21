# handless gameboard - a gaming keyboard for (temporary, I hope) handless dudes
You are soo bored and wanna play games but you've got no left hand? No problem, just use your foot to control some key presses!

![gameboard](gameboard.jpg)

## requirements
- at least one hand
- arduino
- some resistors
- some buttons
- some wires

## how it works
Arduino sends key events to the *gameboard.py* over serial.
These keys are then mapped to profile-specific mappings and propagated to the system.

```sh
$ ./gameboard.py borderlands2
```

## handless gameboard v2: oops I did it AGAIN!!!
Team of our researchers developed more secure implementation of gameboard in Rust language, which deprecated previous version,
Prototype also includes integrated joystick for smooth control.
We are currently evaluating this version, but we dont expect further improvements for really long long long long time.

## disclaimer
this prototype has been wired and programmed using single hand
