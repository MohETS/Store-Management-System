# Labo 0 - BENKHALFA Mohamed-Amine

## Application Description

Cette application est une simple application de ligne de commande qui additionne et soustrait les chiffre `10` et `5` ensemble.

## Project Structure

Le projet est réalisé dans le langage Rust qui offre l'avantage d'être un langage de programmation assez complet. Par exemple, Rust vient avec un très bon linter pris en compte par le compilateur et possède un framework de test qui est directement intégré dans le langage. Cela permet donc de réduire les dépenses, car il n'y a pas un besoin d'avoir une libraire de test externe. Cependant, j'ai rajouté la librairie **Clippy** qui est une libraire qui vient rajouter d'autres lint supplémentaires pour avoir une meilleure couverture.

L'architecture du projet reste très simple, car il s'agit simplement d'une application qui additionne et soustrait 2 chiffres ensemble. Cette logique est aussi réalisée dans un seul fichier `main.rs`. Les tests eux sont exécutés dans un autre fichier (`main_test.rs`) qui se charge uniquement des tests. Il n'y a donc pas beaucoup de choses à dire sur l'architecture pour l'instant.

En ce qui concerne la structure de la pipeline du projet, celle-ci reste simple aussi pour l'instant. Dans le folder `.github/workflows/new_code_action.yml` on peut retrouver le fichier `.yml` qui contient les instruction du GitHub Action qui est exécutée à chaque fois que du nouveau code est poussé sur `main`. Ensuite, nous avons le fichier Dockerfile qui contient les instructions pour la construction de l'image du projet. Finalement, il y a le `docker-compose.yml` qui est le fichier qui permet de lancer l'application en exécutant l'image préalablement créée.

### Github Action (CI/CD)

La pipeline suit le processus recommandé par le laboratoire. Il y a trois jobs qui sont exécuter une après l'autre et chaque job dépends de la réussite de la job précédante pour être exécutée. Si une job ne réussit pas, la pipeline s'arrête là.
L'ordre est le suivant:
1. `lint_job`: Lance la commande cargo clippy qui s'occupe de lancer le linter de Rust
2. `test_job`: Lance la commande `cargo test --verbose` qui s'occupe de lancer les tests automatisés du projet
3. `docker_image_build_job`: Cette job exécute 3 commandes qui sont toutes en lien avec la création et la publication de l'image Docker du projet
   - D'abord la commande `docker build -t ${{secrets.DOCKERHUB_USERNAME}}/${{secrets.DOCKER_IMAGE}} .` est lancé pour créer l'image Docker du projet
   - Ensuite la commande  est lancée pour connecter le processus du Github Action au dépôt Docker Hub
   - Finalement la commande  pousse l'image qui vient d'être créée dans le dépôt Docker Hub

La création de l'image se fait avec le fichier `Dockerfile` qui exécute plusieurs étapes de création d'images afin de créer une image très optimisée pour éviter de créer des images qui peuvent dépasser les 500 Mb voir même 1 Gb. Ensuite une utilisation des variables ***secrets*** du dépôt est utilisée afin de cacher des informations sensibles et d'éviter de les afficher durant l'exécution de la pipeline.

![Successful Github action](/docs/img/GithubAction1.png)

## Execution Instruction

0. Pour pouvoir lancer cette application, il est important de noter qu'il faut avoir préalablement installé Rust : https://www.rust-lang.org/tools/install
1. Lorsque Rust est installé, il faudra cloner le projet
   - `git clone https://github.com/MohETS/Labo0-LOG430.git`
2. Après que le projet ait été cloné et que vous êtes à l'intérieur du dossier du projet vous pouvez exécuter la commande suivante
   - `cargo build`
3. Pour construire l'image Docker
   - `docker build -t labo0/log430 .`
4. Lancer l'image avec Docker Compose
   - `docker compose up` 