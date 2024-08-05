# True Formchange Sora
Skyline plugin that modifies Kingdom Hearts 3 Sora's appearance to switch between Formchanges outfits during certain animations.

## General Changes
* The following list details the Formchanges used and when they're switched into that particular one:
  * **Second**: Down Aerial, Hurricane Blast
  * **Element**: Up Taunt, Firaga, Thundaga, Blizzaga
  * **Strike**: Forward Smash, Up Smash, Down Smash
  * **Guardian**: shielding, Counterattack
  * **Blitz**: ground rolls, air dodge, Dash Attack
  * **Ultima**: Final Smash (will override all other Formchanges while Final Smash is ready)
  * **Light/Dark/Double**: Aerial Sweep
* All standard attacks not listed will utilize the last Formchange switched to (applies to Jab, Tilts, Throws, and Aerials).
* Idle animation will "reset" Formchanges and switch Sora's outfit back to his base outfit.
* Victory animations will use last Formchange switched to.

## What Are Those FLAGs For?
If you look at the source code you'll see multiple FLAG variables being set when a certain Formchange is switched to. This is predominantly basework so that when unique Keyblades are implemented per Formchange, custom effects can easily be switched to during those specific Formchanges. This is still work-in-progress with no ETA on completion.