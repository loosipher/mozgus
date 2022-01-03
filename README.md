# Mozgus

This program is a Discord bot for helping play the game of Call of Cthulhu.
The name comes from a character in the manga *Berserk.* Mozgus is all about
upholding the rules, and so is this bot.

# Configuration

The program does not come with the `config` file, but after being run the
first time, it will panic and generate one. To edit or create said file,
edit/create a file in the root directory called `config`. The first
line is for the Discord token. You must have a bot already made, and giving
it this token will give it access to your server. The second line is for
adjusting the rolls. This is probably left untouched, but my players have
relatively weak characters, so I am adding a feature that will increase
their odds of success by about five percent. The default is `false`, but
you may change this to `true` to increase the rolls' odds of success by
five percent.

# Commands

Mozgus is very much a work in progress. Planned features include NLP
to make rolls happen much more easily, but for now we are just using
commands.

* `/ping` - Mozgus replies with a "Pong!" (mostly for debugging and
testing)
* `/roll` - Roll for a skill.
