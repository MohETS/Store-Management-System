# Tech stack used in this project

### Language

- Rust
- SQL

### Tools

- Docker
- Github

### Rust Libraries

- Cursive
- Diesel

## Justification

* **Cursive**: Cursive a été choisie pour me permettre d’offrir une interface graphique à mon application de console pour la caisse. Cursive a été
  choisie, car elle est compatible avec les OS principaux (Windows, macOS, Linux) grâce à Crossterm. Elle offre aussi un bon niveau d’abstraction pour
  rendre le développement de l’application rapide et simple.


* **Diesel**: J’ai choisi Diesel, car c’est un ORM qui intègre bien les avantages du langage Rust dans son implémentation. Diesel supporte plusieurs bases
  de données, il valide les queries durant la compilation ce qui réduit les erreurs durant le déroulement de l’application. Il utilise aussi le
  système « d’ownership » de Rust ce qui permet d’avoir de bonnes performances et une bonne gestion de la concurrence.