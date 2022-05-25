# PrismaRouter
##### A routing system for sound-active LED traffic.

Some LED systems, (in particular [WLED](https://github.com/Aircoookie/WLED)) allow for LEDs to be sound activated by UDP transmission over the network. This allows one to send the sound output from, say, a computer, to create a set of pulsing LEDs which flash to the beat of music, sound, or what have you. I personally think this effect is **AWESOME**, and sound-active LEDs have long been an interest of mine. The various protocols available which allow this to happen work quite well, but lack just a little bit of extra functionality I desire.

Suppose you have a house like mine, full of ARGB LEDs, and use software like [Lightpack](https://github.com/psieg/Lightpack) to enable sound-active data transmission. But problem: you don't want this data going to just *one* LED strip, you want it going to multiple strips. You could use the network's broadcast address, but this creates a LOT of noise on the network. Further, what if you want to be able to change which strips are active on the fly?

Enter, PrismaRouter. Software designed to route sound-active LED traffic. At its core, all it does is take in UDP traffic that would normally be destined for an LED device, and reroute the traffic verbatim, according to a set of configurable rules.


TODO: literally the rest of this file.
