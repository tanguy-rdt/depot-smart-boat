# depot-smart-boat

![](./.img/gui.gif)

## Usage 

### add your ACCES_KEY

```bash
$ export PICOVOICE_ACCES_KEY="YOUR_ACCES_KEY"
```

### run on MacOS

```bash
$ cargo run 
```

### run on RPI

```bash
$ cargo run --features=on_target
```

## Dected word 

1. wake word: "Ok Bateau"
2. content:
    1. "Tourne/va/allons à gauche"
    2. "Tourne/va/allons à droite"

# TODO 

- camera et combo 
- camera speed 
- bmm150
- potar
- test bateau 
- model 
