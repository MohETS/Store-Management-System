# Title Utilisation de REST pour l'api

## Statut

**Proposed**

## Contexte

Pour faire évoluer l'architecture de notre application de gestion de magasin, nous devons choisir le style de l'architecture de notre API, qui
viendra gérer les communications entre notre client et notre base de données. Notre application est très simple actuellement, mais elle évoluera
encore plus dans les prochaines semaines. Il est donc important de choisir un style d'architecture qui nous offrira le meilleur compromis.

## Décision

Pour la réalisation de notre application, nous avons décidé d'utiliser le style d'architecture REST pour les raisons suivantes

1. Performance
	- Dans le contexte de notre application, REST offre suffisamment de performance pour pouvoir gérer une grande quantité de requêtes sans avoir
	  besoin d'une quantité élevée pour gérer toutes ces ressources. Si on se fie aux benchmarks qui ont été réalisés dans la vidéo **gRPC vs
	  REST vs GraphQL: Comparison & Performance**, on peut voir que pour des situations ou le nombre de requêtes par seconde n'est pas trop élever,
	  REST offre de meilleures performances que gRPC ou GraphQL.

2. Rocket et Simplicité
	- L'utilisation de la librairie Rocket pour la création de notre API nous force à prendre le style d'architecture REST. Cependant, la
	  simplicité qui est offerte avec la librairie Rocket et le style d'architecture REST fait d'elle un bon pour un projet de cette envergure.

## Conséquence

L'architecture REST est un choix idéal pour ce genre d'application et offre suffisamment de marge pour pouvoir augmenter la scalabilité de notre
application dans les futures évolutions. Cependant, après un certain niveau de quantité de requêtes on peut s'attendre à des performances moins bonnes
au fur et à mesure qu'on dépasse ce niveau. Dans des contextes de stress test, cet aspect peut être rapidement visible. Dans un contexte où
l'application nécessitera cette performance pour l'application il sera peut-être nécessaire de changer vers une architecture qui utilise gRPC.

