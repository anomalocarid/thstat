thstat will be a program that allows a player to track their statistics while playing the Touhou Project games. It works by reading information from the game while it is running.

Currently, only basic information (hi-score, score, lives, power, etc.) is supported, and a few games are supported. I plan on supporting all Windows games if possible. thstat is written entirely in the Rust language, and only supports Windows as a target.

# Features

Display the following information in a separate window while playing the game:
* High-score (what is shown in the high-score display)
* Current score
* Current power
* Current life count (number of stars)
* Current bomb count

# Supported Games

* Touhou 10, 11, and 12 are currently supported.

# Acknowledgments

Information about addresses and data types comes from ExpHP's Touhou reverse engineering data repository, which is available at <https://github.com/exphp-share/th-re-data>.