# Taskmaster

Taskmaster est un gestionnaire de processus développé en binôme. L'objectif est de gérer plusieurs programmes définis via un fichier de configuration (lancement, arrêt, redémarrage automatique, monitoring).

## Fonctionnement

Le programme est conçu autour d'une architecture séparant l'interface de contrôle et le moteur d'exécution. Il repose sur deux threads distincts :

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

Voici la section de configuration et la liste des contributeurs intégrées au reste du document.

````markdown
# Taskmaster

Taskmaster est un gestionnaire de processus (job control daemon) développé en binôme. L'objectif est de gérer le cycle de vie de plusieurs programmes définis via un fichier de configuration (lancement, arrêt, redémarrage automatique, monitoring).

## Architecture

Le programme est conçu autour d'une architecture asynchrone stricte séparant l'interface de contrôle et le moteur d'exécution. Il repose sur deux threads distincts :

1. **Thread Main (Prompt / CLI)**
   - Gère l'interface utilisateur interactive.
   - Parse les commandes entrées par l'utilisateur (`start`, `stop`, `status`, `reload`, etc.).
   - Envoie les directives au thread d'exécution.
   - Affiche les retours d'état sans jamais être bloqué par les appels système liés aux processus.

2. **Thread Exec (Exécution et Monitoring)**
   - Fonctionne en arrière-plan comme un daemon.
   - Gère les appels système (`fork`, `execve`, permissions, umask, variables d'environnement, redirections I/O).
   - Surveille en permanence l'état des processus enfants (PID, codes de retour, signaux).
   - Applique les règles de la configuration (restart policy, autostart, expected error codes).

## Synchronisation inter-threads

Les deux threads ne partagent pas d'état mutable. Ils se coordonnent exclusivement via des **channels** (passage de messages).

- Le thread Main envoie des ordres (ex: `ThreadMessage::Start("nom_programme")`).
- Le thread Exec traite l'ordre, interagit avec l'OS, et renvoie un accusé de réception ou un statut (ex: `ThreadMessage::ActionDone`) via un channel de retour.
- Ce modèle garantit que le prompt reste réactif même si un processus géré devient zombie ou prend du temps à s'éteindre.

## Configuration

Le comportement des processus est dicté par un fichier YAML lu au lancement.

### Exemple de `config.yaml`

```yaml
programs:
  ping1:
    cmd: "ping google.com"
    num_processes: 2
    autostart: true
    restart_policy: "UnexpectedExits"
    expected_error_codes:
      - 0
      - 2
    minimum_runtime: 5
    max_relauch_retry: 4
    stop_signal: SIGABRT
    redirect:
      stdout: "ping1_stdout.txt"
      stderr: "ping1_stderr.txt"
    env_to_set:
      chien: "chat"
    umask: 022
  cwd:
    cmd: "./testprogs/cwd/main"
    num_processes: 1
    autostart: true
    restart_policy: "UnexpectedExits"
    expected_error_codes:
      - 0
      - 3
    minimum_runtime: 5
    max_relauch_retry: 4
    redirect:
      stdout: "cwd_stdout.txt"
      stderr: "cwd_stderr.txt"
    env_to_set:
      chien: "chat"
    working_dir: "/tmp"
    umask: 022
```
````

### Paramètres

- `cmd` : Le binaire à exécuter et ses arguments.
- `num_processes` : Nombre d'instances identiques à instancier.
- `autostart` : Démarrage automatique au lancement de Taskmaster (`true` / `false`).
- `restart_policy` : Règle de redémarrage automatique. Valeurs possibles : `Always`, `Never`, `UnexpectedExits`.
- `expected_error_codes` : Liste des codes de retour considérés comme une fin d'exécution normale.
- `minimum_runtime` : Temps (en secondes) pendant lequel le processus doit rester actif pour considérer que son lancement a réussi.
- `max_relauch_retry` : Nombre maximal de tentatives de redémarrage consécutives en cas d'échec.
- `stop_signal` : Signal UNIX utilisé pour demander l'arrêt propre du processus (ex: `SIGTERM`, `SIGABRT`).
- `redirect` : Chemins des fichiers pour la redirection des flux `stdout` et `stderr`.
- `env_to_set` : Variables d'environnement à injecter dans le contexte d'exécution du processus.
- `working_dir`: Répertoire courant (CWD) à appliquer avant l'exécution du binaire.
- `umask` : Masque de création de fichiers en valeur octale (ex: `022`).

## Compilation et Exécution

```bash
cargo build --release
./target/release/taskmaster path/to/config.yaml
```

## Contributeurs

- [hfragnoli](https://github.com/hugofragnoli)
- [maecarva](https://github.com/maecarva)
