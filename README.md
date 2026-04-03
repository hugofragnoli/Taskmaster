# Taskmaster

Taskmaster est un gestionnaire de processus développé en binôme. L'objectif est de gérer le cycle de vie de plusieurs programmes définis via un fichier de configuration (lancement, arrêt, redémarrage automatique, monitoring).

## Architecture

Le programme est conçu autour d'une architecture asynchrone séparant l'interface de contrôle et le moteur d'exécution. Il repose sur deux threads distincts :

1. **Thread Main (Prompt / CLI)**
   - Gère l'interface utilisateur interactive.
   - Parse les commandes entrées par l'utilisateur (`start`, `stop`, `status`, `reload`, etc.).
   - Envoie les directives au thread d'exécution.
   - Affiche les retours d'état sans jamais être bloqué par les appels système liés aux processus.

2. **Thread Exec (Exécution et Monitoring)**
   - Fonctionne en arrière-plan comme un daemon.
   - Démarre les processus (permissions, umask, variables d'environnement, redirections I/O).
   - Surveille en permanence l'état des processus enfants (PID, codes de retour, signaux).
   - Applique les règles de la configuration (restart policy, autostart, expected error codes).

## Synchronisation inter-threads

Les deux threads se coordonnent exclusivement via des **channels** (passage de messages). 

- Le thread Main envoie des ordres (ex: `ThreadMessage::Start("nom_programme")`).
- Le thread Exec traite l'ordre, interagit avec l'OS, et renvoie un accusé de réception ou un statut (ex: `ThreadMessage::ActionDone`) via un channel de retour.
- Ce modèle garantit que le prompt reste réactif même si un processus géré devient zombie ou prend du temps à s'éteindre.

## Compilation et Exécution

```bash
cargo build --release
./target/release/taskmaster path/to/config.yaml
```
