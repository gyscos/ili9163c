This is a rust library to control a [ILI9163C LCD module].

It is meant for microcontrollers (it doesn't require std), but can also be used on a regular computer.

You will need to give it a [SPI] device (implement the trait for your struct) and two [Pin]s.

To debug it on your computer, you may want to use the [ili9163c_simulator] library.

[ILI9163C LCD module]: https://www.google.com/search?q=ILI9163C&tbm=isch
[SPI]: https://github.com/gyscos/ili9163c/blob/master/src/spi.rs#L6
[Pin]: https://github.com/gyscos/ili9163c/blob/master/src/gpio.rs#L2
[ili9163c_simulator]: https://github.com/gyscos/ili9163c_simulator
