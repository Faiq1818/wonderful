# Wonderful

Launcher for productivity that can launch many thing at one times, so you can make many items for your projects, build in rust.

![](https://github.com/Faiq1818/wonderful/blob/main/assets/demo.gif)

### Installing

Clone this repository

    git clone https://github.com/Faiq1818/wonderful.git

Go inside the folder

    cd wonderful
    
Build

    cargo install --path .

Run the program

    wonderful

### Hotkeys

| Hotkey       | Description         |
|--------------|---------------------|
| `Esc`        | Quit application    |
| `↓` or `Tab` | Move down           |
| `↑` or `Shift + Tab` | Move up             |
| `Enter`      | Select              |
| `Other key`  | Typing in Find      |

### Documentation

A TOML file will be automatically created inside .config/wonderful when you open this app for the first time.  
You can create multiple tables and their key-value pairs, and they will be scanned every time you open the Wonderful app.  
#### Example:
```
["Open project x"]
"open firefox" = "firefox"
vscode = "code"

[OBS]
"Open OBS" = "obs"
```
##### Rules:
1. You can use square brackets for table names. If the name contains more than one word, wrap it in double quotes.
2. Keys must be in double quotes if they contain more than one word.
3. Values must always be in double quotes.
4. The key name doesn't matter. The program will execute all values in the selected table when you press Enter.

## Contributing

Contributions are open to everyone.  
Feel free to fork the repository, make changes, and submit a pull request.  
There is no strict format — just make sure the code is clean and complies with the license.


## License

This project is licensed under the [GPLv3](LICENSE)
