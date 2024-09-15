# SDAT/SF2 ADSR Envelope Calculator

A small app to calculate envelopes for use in editing SDAT files for NDS roms. It can also go the other way around, converting the 8-bit envelope values that SDAT uses to accurate* time values for soundfont creation.

I referred primarily to Kermalis' [VGMusicStudio](https://github.com/Kermalis/VGMusicStudio/) NDS Core for the tables and math, alongside VGMTrans' SF2 output as well as Nitro Studio 2's output and my own judgement.

*I have no way of verifying they're completely accurate other than using my ears, and it SEEMS accurate enough but my math and algorithms might be wrong.

### How to use

Type in the desired values into the Attack, Decay, Sustain, and Release text boxes.

If any decimal values or values other than integers 0-127 are entered, a conversion from SDAT to SF2 is not possible.

If and only if all fields are filled with an integer 0-127, the calculate button will allow conversion to SF2.

If an invalid number is entered, the field will use its default value.