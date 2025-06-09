# Utilisation de PostgreSQL comme base de données

## Statut

**Proposed**

## Contexte

Dans le cadre du laboratoire du cours LOG430, nous devons créer une application qui évoluera architecturalement à travers la réalisation de plusieurs
laboratoires. L’application s’agira d’un système de caisse qui lui aussi évoluera en ce qui concerne le scope de l’application. Elle deviendra un
système multi succursale et potentiellement un e-commerce distribué

## Décision

Nous avons décidé de choisir PostgreSQL pour la base de données des laboratoires pour les raisons suivantes :

1. Contexte des données et relations
	- Dans le contexte d’un système de caisse, les différentes données que l’on peut avoir ont des relations clairement établies comme une vente qui
	  est formée de plusieurs produits par exemple. Prendre une base de données relationnelle offre donc une meilleure gestion de ces cas.

2. ACID
	- Puisque notre système fonctionne dans un contexte financier, nous avons besoin de données qui sont fiables. Comme PostgreSQL respecte les
	  critères ACID, on peut être sûr que nos données resteront fiables avec les multiples transactions qu’il y aura.

## Conséquence

La création des différentes tables nécessaires pour le fonctionnement de la base de données durant le développement de l’application requiert plus de
temps que la création de schémas de document dans une approche NoSQL