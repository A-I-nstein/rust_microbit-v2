# rust_microbit-v2
A Rust implementation of the "Meet the micro:bit" introductory program for the BBC micro:bit-v2. Link: [Meet the microbit program](https://microbit.org/get-started/user-guide/meet-the-microbit-program/)

## "Meet the micro:bit" Program Features

### [X] Welcome Animation
When first powered on, the LED display will usually light up with a welcoming animation, often a heart, and play a welcome tune.

### [ ] Button Interaction (A and B)
The program will prompt you to press button A, then button B, and sometimes both A and B together.
Each press triggers a different response on the 5x5 LED matrix, such as showing different icons, animations (like a square zooming in and out), or scrolling text. This demonstrates the basic input capability of the buttons.

### [ ] Accelerometer (Motion Sensing)
* **Shake Detection:** The program will ask you to shake the micro:bit. The harder you shake it, the more LEDs might light up, or a specific animation (like a surprised face) will appear. This showcases the accelerometer's ability to detect motion and force.
* **Tilt Game:** Some versions include a simple game where you need to tilt the micro:bit to get two dots on the LED display to meet. This further demonstrates the accelerometer's ability to sense orientation.

### [ ] Light Sensor Demonstration (using the LED matrix)
The program might include a segment where pressing buttons A and B together reveals a puzzle or different images (like a sun or a moon) based on the ambient light level.
This shows that the LED display itself can also function as a basic light sensor. The program uses an "if...then...else" conditional logic to display different images depending on whether the light level is above or below a certain threshold.

### [ ] Sound Output (Speaker - V2 specific)
For micro:bit V2 devices, the pre-installed program will often play different sounds or musical notes in response to the various inputs (button presses, shakes). This highlights the built-in speaker.

### [ ] Microphone Input (V2 specific)
Some versions of the "Meet the micro:bit" program for V2 might include an activity that reacts to sound, like counting claps. This demonstrates the capability of the built-in microphone.

### [ ] Touch-Sensitive Logo (V2 specific)
The program may guide you to touch the gold micro:bit logo. Releasing the logo after saying a few words, or simply touching it, can trigger a response on the LEDs or a sound, showcasing this additional input method.

### [ ] Easter Egg/Secret Game
Older versions of the "Out of Box Experience" (and some "Meet the micro:bit" versions) include a hidden "snake" game. This is often accessed by pressing buttons A and B together when a specific animation (like a heart) is displayed.

### [ ] Power Saving Mode (V2 specific)
While not an interactive part of the game, the ability to put the micro:bit V2 into a power-saving sleep mode by holding down the reset button on the back (until the power LED fades) is a feature that the user might discover or be guided to during their initial exploration. Pressing the reset button again wakes it up.
