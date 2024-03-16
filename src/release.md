# Création d'une nouvelle release 

Plusieurs solutions s'offrent à vous si vous avez réalisé des modifications et que vous souhaitez compiler un nouveau package d'application.

## Pour debian  &#8594; `.deb`
### Sur debian 
Cette méthode est simple, il suffit d'exécuter un script qui réalisera pour vous un `.deb` qui est placé après compilation dans le dossier `./deb`.

```bash
$ ./package-builder/debian/bd_smart-boat-gui_debian.sh
```

_Cependant, si vous ne développez pas sur Linux alors il faut utiliser les méthodes suivantes._

### Sur un OS différent avec Docker

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

## Pour Debian et MacOS avec GitHub action 

Nous avons mis en place un workflow qui permet de compiler l'application et d'en réaliser un package en mode natif et stub. \

Pour que ce workflow se lance, il vous suffit de pousser vos modifications avec un tag qui débute par _'v'_ suivie d'un numéro de version, par exemple _v1.0.0_. \
L'action GitHub va alors se lancer et après sont succès vous pouvez trouver tous les packages dans une nouvelle [release](https://github.com/tanguy-rdt/depot-smart-boat/releases) qui porte comme nom la version du tag que vous avez utilisé.


