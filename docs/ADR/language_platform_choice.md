# Utilisation de Rust comme langage/platforme pour la création de l'application

## Statut

**Proposed**

## Contexte

Dans le cadre du laboratoire du cours LOG430, nous devons créer une application qui évoluera architecturalement à travers la réalisation de plusieurs
laboratoires. Puisque c’est un projet à réaliser dans le cadre d’un cours, il faut
aussi une plate-forme qui offre assez d’outils pour gérer la création d’une application distribuée.

## Décision

Nous avons décidé de choisir Rust comme langage pour la réalisation des laboratoires pour les raisons suivantes :

1. Concurrence
	- Avec la gestion bas niveau de la mémoire et le système « d’ownership » qu’offre Rust, nous avons beaucoup d’outils qui nous aide à gérer
	  l’aspect de concurrence dans la création de notre application.

2. Performance
	- Rust étant considéré comme un langage de « bas niveau », il offre une performance très élevée. Puisque Rust est un langage compilé et
	  qu’il offre un contrôle de bas niveau sur la mémoire, on peut obtenir des performances qui sont très utiles dans un contexte où l’on crée
	  une application qui doit gérer beaucoup de requêtes.

3. Écosystème
	- Rust propose aujourd’hui plusieurs librairies qui offrent des outils très utiles pour la création d’application distribuée comme ***Tokio***
	  pour l’aspect asynchrone d’une application, ***Diesel*** pour les interactions avec une base de données et Rocket pour créer des applications.
	  Tous ces outils offriront une bonne base d’outils pour la création de notre application tout en gardant les avantages de Rust

## Conséquence

Le choix de Rust vient avec plusieurs avantages technologiques, mais la courbe d’apprentissage peut être grande, car nous ne sommes pas des
experts en Rust. Rust peut aussi rendre la création initiale du projet plus lente que des langages de programmation de haut niveau, ce qui
pourrait ralentir notre développement dans les premiers laboratoires.
