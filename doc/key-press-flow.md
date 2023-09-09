# What Happens When You Press a Key in Zellij (Assuming the Terminal Emulator is Alacritty and the Shell is zsh)
Alacritty, as a terminal emulator, provides a "pseudo-terminal" (pty), which simulates a physical terminal from the past. This pty has two sides: the master (ptm) and the slave (pts or pty).

- The master side provides an interface for the terminal emulator, allowing it to send input to and receive output from the slave side.
- The slave side provides an interface for programs that wish to connect to a terminal, such as your shell (in this case, zsh).

So, when you open Alacritty, it creates a pty pair, connects the ptm to itself, and starts a shell connected to the pts.

When you start Zellij:

1. It connects to the pts provided by Alacritty(like other normal app)
2. For each pane you open, Zellij creates a new pty pair. It connects the ptm to itself and starts a shell connected to the pts.

Therefore, when you have 4 panes open in Zellij, there are a total of 5 pty pairs: one for the connection between Alacritty and Zellij, and one for each pane. Each ptm is connected to either Alacritty or Zellij, and its pts is connected to either Zellij or a shell. In this way, Zellij acts as a middleman, controlling input and output between Alacritty and the shell.

When you press a key in the terminal:
1. From the keyboard to the terminal emulator (Alacritty): You press a key on the keyboard, and then the operating system sends a message to Alacritty, telling it which key has been pressed.
2. From Alacritty to Zellij: Alacritty receives this input and sends it to the master side of its pty. Then, Zellij receives this data on the slave side of the pty.
3. From Zellij to the current active shell: Zellij has its own set of ptys for each pane. It reads the input from Alacritty, processes it, and then sends it to the master side of the pty for the current active pane. The shell (such as zsh) is connected to the slave side of this pty, so it receives this input.
4. From the shell to the command: The shell parses the received input as a command or part of a command and executes it.

In Zellij, Zellij keeps track of the "current" pane, and the keypress eventually gets sent to the pts corresponding to the current pane.
