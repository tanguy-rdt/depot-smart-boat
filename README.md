# Smart Boat 

> Melvin DUBEE
> Tanguy ROUDAUT
> 
> ENSTA Bretagne

[![SmartBoat GUI Build and Release](https://github.com/tanguy-rdt/depot-smart-boat/actions/workflows/main.yml/badge.svg)](https://github.com/tanguy-rdt/depot-smart-boat/actions/workflows/main.yml)
[![mdbook build-deploy](https://github.com/tanguy-rdt/depot-smart-boat/actions/workflows/gh-pages.yml/badge.svg)](https://github.com/tanguy-rdt/depot-smart-boat/actions/workflows/gh-pages.yml)
[![pages-build-deployment](https://github.com/tanguy-rdt/depot-smart-boat/actions/workflows/pages/pages-build-deployment/badge.svg)](https://github.com/tanguy-rdt/depot-smart-boat/actions/workflows/pages/pages-build-deployment)

Ce projet met en œuvre un système intelligent intégrant une interface graphique basé sur [_egui_](https://github.com/emilk/egui) et une commande vocale pour le contrôle d'une maquette de bateau. Différents capteurs de navigation et des moteurs permettent de simuler une navigation en sécurité et assisté.

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

## Création d'une nouvelle release 

Plusieurs solutions s'offrent à vous si vous avez réalisé des modifications et que vous souhaitez compiler un nouveau package d'application.

### Pour debian  &#8594; `.deb`
#### Sur debian 
Cette méthode est simple, il suffit d'exécuter un script qui réalisera pour vous un `.deb` qui est placé après compilation dans le dossier `./deb`.

```bash
$ ./package-builder/debian/bd_smart-boat-gui_debian.sh
```

_Cependant, si vous ne développez pas sur Linux alors il faut utiliser les méthodes suivantes._

#### Sur un OS différent avec Docker

Nous avons mis en place un conteneur docker pour réaliser un package compatible sur debian en mode natif et stub depuis un OS de compilation différent. \

Pour utiliser docker il vous suffit:
1. D'installer docker 
2. Compilation de l'image docker
    ```bash
    $ ./package-builder/debian/docker/bd_docker-img_debian.sh
    ```
3. Lancement du conteneur 
    ```bash
    $ ./package-builder/debian/run_docker-img_debian.sh
    ```

Après exécution du conteneur, vous pouvez trouver le fichier `.deb` dans le dossier `./deb`. Le docker exécute simplement un conteneur debian et le script que vous auriez exécuté si vous utilisez un OS de compilation Linux. 

### Pour Debian et MacOS avec GitHub action 

Nous avons mis en place un workflow qui permet de compiler l'application et d'en réaliser un package en mode natif et stub. 

Pour que ce workflow se lance, il vous suffit de pousser vos modifications avec un tag qui débute par _'v'_ suivie d'un numéro de version, par exemple _v1.0.0_. \
L'action GitHub va alors se lancer et après sont succès vous pouvez trouver tous les packages dans une nouvelle [release](https://github.com/tanguy-rdt/depot-smart-boat/releases) qui porte comme nom la version du tag que vous avez utilisé.


