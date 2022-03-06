## General information

_Calculate42_ is a text based game oriented on messengers, which can perform functions of calculator. The game is a mathematical puzzle with elements of a social puzzle. Player’s primary aim write an expression that gives a number “42”. But AI antagonist, living in the calculator hinders the player.

## Features

- Calculations of mathematical expressions including (for example: 2 + 2 * (2 - 2) / 2):
  - Addition +
  - Subtraction -
  - Multiplication *
  - Division /
  - Integer division //
  - Reminder of division %
  - Brackets ()
  - Exponentiation **
- Conversations with AI
- The game is divided into rounds in each of which the player needs to get "42".
- A story that can be learned through conversations and gameplay.

## Detailed description

_Calculate42_ is a state machine that changes its state every time it receives a message from the player. It replays to the player every time that it’s getting the message. The reply is the solution to an expression, line of the AI or message with the state of the game.

### Calculations

_Calculate42_ calculates any expession with white space or without sended to it in any time and any state of gameplay or a conversation. Expressions which will be calculated:

- 2 + 2
- 2+2
- 2 * (2+ 2)
- 2 /2**4 * 1490

Expressions which will not be calculated:

- 2 2
- 2 * (2 + ( 2)
- 2 - + 8

_Calculate42_ will send the answer at the same moment it receives and calculates the expression.