# Smart Boat 

> Melvin DUBEE
> Tanguy ROUDAUT
> 
> ENSTA Bretagne

[![SmartBoat GUI Debian](https://github.com/tanguy-rdt/depot-smart-boat/actions/workflows/smart-boat_debian.yml/badge.svg)](https://github.com/tanguy-rdt/depot-smart-boat/actions/workflows/smart-boat_debian.yml)

Ce projet met en ouvre un système intelligent intégrant une interface graphique et une commande vocale pour le contrôle d'une maquette de bateau. Différent capteurs de naviguation et des moteurs permettent de simuler une naviguation en sécurité et assisté.

Une documentation plus détailé est disponnible [ici](https://tanguy-rdt.github.io/depot-smart-boat/).


## Usage 

### Installation de rust

Le code de ce projet est réalisé en rust, il est donc important de l'avoir installé.

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Ajouter votre clé d'accès Picovoice

Vous pouvez ajouter votre clé d'accès:
- De manière temporaire dans le terminal actif avec `export PICOVOICE_ACCES_KEY="YOUR_ACCES_KEY"`
- De manière permanente en ajoutant cette commande dans votre `~/.bashrc` ou `~/.zshrc` em fonction du shell utilisé.

_Si vous n'avez pas de clé d'accès vous pouvez en obtenir une en créant un compte sur le site de [picovoice](https://console.picovoice.ai/login) gratuitement._



### Sur RaspberryOS

Cette commande marchera uniquement avec un Raspberry Pi ou un émulateur qui prend en charge les ports GPIO. Le mode sur cible prend en compte la crate rppal qui permet de contrôler les ports GPIO.

```bash
$ cargo run --features=on_target
```

Actuellement l'interface graphique en WASM n'a pas était réalisé. Vous devez connecter un écran au Raspberry Pi ou utiliser un serveur VNC _(le serveur doit être activé avec la commande `sudo raspi-config` puis activer le serveur VNC dans le menu `Interface`)._


### Sur un OS différent de la cible _(Mode stub)_

Le mode _stub_ vous permet d'éxecuter l'interface graphique avec une maquette simulé pour s'affranchire de capteurs et des GPIO. Cependant vous-devez quand même avoir enregistré votre clé d'accès Picovoice. 

Cela est particulièrement utile pour réaliser des modifications sur l'interface graphique.

```bash
$ cargo run 
```

## Mots détecté avec la commande vocale

1. wake word: "Ok Bateau"
2. content:
    - "Tourne/va/allons à gauche/babord"
    - "Tourne/va/allons à droite/tribord"
    - "vent de près à gauche/babord"
    - "vent de près à droite/tribord"
    - "vent de face"
    - "vent arrière" 
    - "vent largue à gauche/babord"
    - "vent largue à droite/tribord"
    - "vent de travers à gauche/babord"
    - "vent de travers à droite/tribord"

