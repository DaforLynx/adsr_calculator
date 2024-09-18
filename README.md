# SDAT/Sappy/SF2 ADSR Envelope Calculator

A small app to calculate envelopes for use in editing SDAT files for NDS roms or entering envelope values into Sappy (MP2K) manifests for GBA roms. It can also go the other way around, converting the 8-bit envelope values that SDAT uses to accurate* time values for soundfont creation.

I referred primarily to Kermalis' [VGMusicStudio](https://github.com/Kermalis/VGMusicStudio/) NDS Core for the tables and math, alongside [VGMTrans](https://github.com/vgmtrans/vgmtrans)' SF2 output as well as Nitro Studio 2's output and my own judgement.

*I have no way of verifying they're completely accurate other than using my ears, and it SEEMS accurate enough but my math and algorithms might be wrong.

### How to use

Type in the desired values into the Attack, Decay, Sustain, and Release text boxes.

If any decimal values or values other than integers 0-255 are entered, a conversion to SF2 is not possible.

If and only if all fields are filled with an integer 0-127 (NDS) or 0-255 (GBA), the calculate button will allow conversion to SF2.

If an invalid number is entered, the field will use its default value.

If a result is displayed, you can press "Copy to clipboard" to copy it to your clipboard in order to paste directly in Polyphone, or a spreadsheet.

You can also copy values out of Nitro Studio 2's bank editor or a voicegroup.inc file and paste them in this app directly by pressing "Paste from clipboard" or pasting into a text box. Values are split along commas and whitespace, with extra code to handle copying from Polyphone.