> Melvin DUBEE - Tanguy ROUDAUT
> 
> _ENSTA Bretagne_

Ce projet met en œuvre un système intelligent intégrant une interface graphique basé sur [_egui_](https://github.com/emilk/egui) et une commande vocale pour le contrôle d'une maquette de bateau. Différents capteurs de navigation et des moteurs permettent de simuler une navigation en sécurité et assisté.


## Usage 

### Installation de rust

Le code de ce projet est réalisé en rust, il est donc important de l'avoir installé.

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Ajouter votre clé d'accès Picovoice

Vous pouvez ajouter votre clé d'accès:
- De manière temporaire dans le terminal actif avec `export PICOVOICE_ACCES_KEY="YOUR_ACCES_KEY"`
- De manière permanente en ajoutant cette commande dans votre `~/.bashrc` ou `~/.zshrc` en fonction du shell utilisé.

_Si vous n'avez pas de clé d'accès, vous pouvez en obtenir une en créant un compte sur le site de [picovoice](https://console.picovoice.ai/login) gratuitement._



### Sur RaspberryOS

Cette commande marchera uniquement avec un Raspberry Pi ou un émulateur qui prend en charge les ports GPIO. Le mode sur cible prend en compte la crate rppal qui permet de contrôler les ports GPIO.

```bash
$ cargo run --features=on_target
```

Actuellement l'interface graphique en WASM n'a pas était réalisé. Vous devez connecter un écran au Raspberry Pi ou utiliser un serveur VNC _(le serveur doit être activé avec la commande `sudo raspi-config` puis activer le serveur VNC dans le menu `Interface`)._


### Sur un OS différent de la cible _(Mode stub)_

Le mode _stub_ vous permet d'exécuter l'interface graphique avec une maquette simulée pour s'affranchir de capteurs et des GPIO. Cependant vous devez quand même avoir enregistré votre clé d'accès Picovoice. 

Cela est particulièrement utile pour réaliser des modifications sur l'interface graphique.

```bash
$ cargo run 
```

## Mots détectés avec la commande vocale

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


